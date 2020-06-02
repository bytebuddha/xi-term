use tui::layout::Rect;
use xrl::{LineCache, Update, ConfigChanges};

use super::{ client::Client, window::Window };

#[derive(Debug, Default)]
pub struct Cursor {
    pub line: u64,
    pub column: u64,
}

pub struct View {
    pub cache: LineCache,
    cursor: Cursor,
    pub window: Window,
    pub client: Client,
    cfg: Option<ConfigChanges>,
    pub pristine: bool,
    pub rect: Option<Rect>
}

impl View {
    pub fn new(client: Client) -> View {
        View {
            cache: LineCache::default(),
            cursor: Default::default(),
            window: Window::new(),
            cfg: None,
            client,
            pristine: false,
            rect: None
        }
    }

    pub fn update_cache(&mut self, update: Update) {
        info!("updating cache");
        self.pristine = update.pristine;
        self.cache.update(update);
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

    pub fn config_changed(&mut self, changes: ConfigChanges) {
        self.cfg = Some(changes);
    }

    pub fn render_cursor(&self, area: Rect) -> Option<(u16, u16)> {
        info!("rendering cursor");
        if self.cache.is_empty() {
            trace!("cache is empty, rendering cursor at the top left corner");
            return None;
        }

        if (self.cursor.line) < self.cache.before() {
            warn!(
                "the cursor is on line {} render_cursor which is marked invalid in the cache",
                self.cursor.line
            );
            return None;
        }
        // Get the line that has the cursor
        //if self.cursor.line < area.y as u64 {
        //    warn!("the cursor line {} is not within the visible rect {:?}", self.cursor.line, area);
        //    return None;
        //}
        let line_idx = (self.cursor.line) - self.cache.before();
        let line = match self.cache.lines().get(line_idx as usize) {
            Some(line) => line,
            None => {
                warn!("no valid line at cursor index {}", self.cursor.line);
                return None;
            }
        };

        if line_idx < (self.window.start()) {
            warn!(
                "the line that has the cursor (nb={}, cache_idx={}) not within the displayed window ({:?})",
                self.cursor.line + area.y as u64,
                line_idx,
                self.window
            );
            return None;
        }
        // Get the line vertical offset so that we know where to draw it.
        let line_pos = line_idx - self.window.start() + area.y as u64;

        // Calculate the cursor position on the linerender_cursor. The trick is that we know the position within
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
        Some((column as u16 + area.x + 1, line_pos as u16))
    }

    fn translate_char_width(&self, position: u16, c: char) -> u16 {
        match c {
            // Caret notation means non-tab control characters are two columns wide
            '\x00'..='\x08' | '\x0a'..='\x1f' | '\x7f' => 2,
            '\t' => self.get_tab_size(position),
            _ => 1,
        }
    }

    fn get_tab_size(&self, position: u16) -> u16 {
        if let Some(cfg) = &self.cfg {
            if let Some(tab_size) = &cfg.tab_size {
                let tab_size = *tab_size as u16;
                return tab_size - (position % tab_size);
            }
        }
        4 - (position % 4)
    }

    pub fn save(&mut self, file: &str) {
        self.client.save(file);
    }

    pub fn click(&mut self, rect: Rect, x: u64, y: u64) {
        let (line, column) = self.get_click_location(rect, x, y);
        error!("CLick Location: {}, {}", line, column);
        self.client.click(line, column);
    }

    /// TODO: Fix this it is very buggy
    fn get_click_location(&self, rect: Rect, x: u64, y: u64) -> (u64, u64) {
        let lineno = if x + self.cache.before() + self.window.start() < rect.y as u64 {
            x + self.cache.before() + self.window.start()
        } else {
            x + self.cache.before() + self.window.start() - rect.y as u64
        };
        if let Some(line) = self.cache.lines().get(x as usize) {
            if y < 5 {
                return (lineno, rect.y as u64);
            }
            let mut text_len: u16 = 0;
            for (idx, c) in line.text.chars().enumerate() {
                let char_width = self.translate_char_width(text_len, c);
                text_len += char_width;
                if u64::from(text_len) >= y {
                    // If the character at idx is wider than one column,
                    // the click occurred within the character. Otherwise,
                    // the click occurred on the character at idx + 1
                    if char_width > 1 {
                        return (lineno, (idx - rect.x as usize) as u64 - rect.y as u64);
                    } else {
                        return (
                            lineno as u64,
                            (idx - rect.x as usize) as u64,
                        );
                    }
                }
            }
            return (lineno - rect.x as u64, line.text.len() as u64 + 1);
        } else {
            warn!("no line at index {} found in cache", x);
            return (x + rect.x as u64, y);
        }
    }
}
