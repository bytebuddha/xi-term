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
    pub cursor: Cursor,
    pub window: Window,
    pub client: Client,
    pub cfg: Option<ConfigChanges>,
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

    pub fn translate_char_width(&self, position: u16, c: char) -> u16 {
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
