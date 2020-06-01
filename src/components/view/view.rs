use termion::event::{Event, Key};
use xrl::{LineCache, Update};

use super::client::Client;
use super::window::Window;

#[derive(Debug, Default)]
pub struct Cursor {
    pub line: u64,
    pub column: u64,
}

pub struct View {
    pub cache: LineCache,
    cursor: Cursor,
    pub window: Window,
    client: Client
}

impl View {
    pub fn new(client: Client) -> View {
        View {
            cache: LineCache::default(),
            cursor: Default::default(),
            window: Window::new(),
            client
        }
    }

    pub fn update_cache(&mut self, update: Update) {
        info!("updating cache");
        self.cache.update(update)
    }

    pub fn set_cursor(&mut self, line: u64, column: u64) {
        self.cursor = Cursor { line, column };
        self.window.set_cursor(&self.cursor);
    }


    pub fn resize(&mut self, height: u16) {
        self.window.resize(height);
        self.update_window();
        let top = self.cache.before() + self.window.start();
        let bottom = self.cache.after() + self.window.end();
        self.client.scroll(top, bottom);
    }

    pub fn insert(&mut self, c: char) {
        self.client.insert(c)
    }

    pub fn insert_newline(&mut self) {
        self.client.insert_newline()
    }

    pub fn insert_tab(&mut self) {
        self.client.insert_tab()
    }

    fn update_window(&mut self) {
        if self.cursor.line < self.cache.before() {
            error!(
                "cursor is on line {} but there are {} invalid lines in cache.",
                self.cursor.line,
                self.cache.before()
            );
            return;
        }
        let cursor_line = self.cursor.line - self.cache.before();
        let nb_lines = self.cache.lines().len() as u64;
        self.window.update(cursor_line, nb_lines);
    }

    pub fn handle_input(&mut self, event: Event) {
        match event {
            Event::Key(key) => match key {
                Key::Char(c) => match c {
                    '\n' => self.insert_newline(),
                    '\t' => self.insert_tab(),
                    _ => self.insert(c),
                },
                Key::Ctrl(c) => match c {
                    'h' => self.client.backspace(),
                    _ => error!("un-handled input ctrl+{}", c),
                },
                Key::Backspace => self.client.backspace(),
                Key::Delete => self.client.delete(),
                Key::Left => self.client.left(),
                Key::Right => self.client.right(),
                Key::Up => self.client.up(),
                Key::Down => self.client.down(),
                Key::Home => self.client.home(),
                Key::End => self.client.end(),
                Key::PageUp => self.client.page_up(),
                Key::PageDown => self.client.page_down(),
                k => error!("un-handled key {:?}", k),
            },
            ev => error!("un-handled event {:?}", ev),
        }
    }

    pub fn render_cursor(&self) -> (u16, u16) {
        info!("rendering cursor");
        if self.cache.is_empty() {
            trace!("cache is empty, rendering cursor at the top left corner");
            return (1, 1);
        }

        if self.cursor.line < self.cache.before() {
            warn!(
                "the cursor is on line {} which is marked invalid in the cache",
                self.cursor.line
            );
            return (1, 1);
        }
        // Get the line that has the cursor
        let line_idx = self.cursor.line - self.cache.before();
        let line = match self.cache.lines().get(line_idx as usize) {
            Some(line) => line,
            None => {
                warn!("no valid line at cursor index {}", self.cursor.line);
                return (1, 1);
            }
        };

        if line_idx < self.window.start() {
            warn!(
                "the line that has the cursor (nb={}, cache_idx={}) not within the displayed window ({:?})",
                self.cursor.line,
                line_idx,
                self.window
            );
            return (1, 1);
        }
        // Get the line vertical offset so that we know where to draw it.
        let line_pos = line_idx - self.window.start();

        // Calculate the cursor position on the line. The trick is that we know the position within
        // the string, but characters may have various lengths. For the moment, we only handle
        // control characters and tabs. We assume control characters (0x00-0x1f, excluding 0x09 ==
        // tab) are rendered in caret notation and are thus two columns wide. Tabs are
        // variable-width, rounding up to the next tab stop. All other characters are assumed to be
        // one column wide.
        let column: u16 = line
            .text
            .chars()
            .take(self.cursor.column as usize)
            .fold(0, |acc, c| acc + self.translate_char_width(acc, c));

        (column, line_pos as u16)
    }

    fn translate_char_width(&self, position: u16, c: char) -> u16 {
        match c {
            // Caret notation means non-tab control characters are two columns wide
            '\x00'..='\x08' | '\x0a'..='\x1f' | '\x7f' => 2,
            '\t' => 4 - (position % 4),
            _ => 1,
        }
    }
}
