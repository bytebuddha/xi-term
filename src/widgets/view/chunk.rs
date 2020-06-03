use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;
use tui::style::{ Style, Color, Modifier };
use xrl::ThemeChanged;

pub struct Chunk<'a, 'b> {
    text: &'a str,
    background: Option<Color>,
    foreground: Option<Color>,
    italic: Option<bool>,
    underlined: Option<bool>,
    theme: Option<&'b ThemeChanged>
}

impl <'a, 'b>Chunk<'a, 'b> {

    pub fn new(text: &'a str) -> Chunk<'a, 'b> {
        Chunk {
            text,
            background: None,
            foreground: None,
            italic: None,
            underlined: None,
            theme: None
        }
    }

    pub fn foreground(mut self, c: Option<Color>) -> Chunk<'a, 'b> {
        self.foreground = c;
        self
    }

    pub fn background(mut self, c: Option<Color>) -> Chunk<'a, 'b> {
        self.background = c;
        self
    }

    pub fn underlined(mut self, c: Option<bool>) -> Chunk<'a, 'b> {
        self.underlined = c;
        self
    }

    pub fn italic(mut self, c: Option<bool>) -> Chunk<'a, 'b> {
        self.italic = c;
        self
    }

    pub fn theme(mut self, theme: Option<&'b ThemeChanged>) -> Chunk<'a, 'b> {
        self.theme = theme;
        self
    }
}

impl <'a, 'b>Widget for Chunk<'a, 'b> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.text.is_empty() {
            if let Some(background) = self.background {
                buf.set_background(area, background);
            }
            return;
        }
        let mut data = String::new();
        // This Variable is undefined? WTF
        #[allow(unused_variables)]
        let mut position: u16 = 0;
        for chr in self.text.chars() {
            match chr {
                '\x00'..='\x08' | '\x0a'..='\x1f' | '\x7f' => {
                    // Render in caret notation, i.e. '\x02' is rendered as '^B'
                    data.push('^');
                    data.push((chr as u8 ^ 0x40u8) as char);
                    position += 2;
                }
                '\t' => {
                    data.push_str(&" ".repeat(4));
                    position += 4;
                }
                _ => {
                    data.push(chr);
                    position += 1;
                }
            }
        }

        buf.set_stringn(
            area.x,
            area.y,
            &data,
            data.len(),
            get_style(&self)
        );
    }

}

fn get_style<'a, 'b>(chunk: &Chunk<'a, 'b>) -> Style {
    let mut style = Style::default();

    if let Some(background) = chunk.background {
        style.bg = background;
    } else if let Some(theme) = chunk.theme {
        if let Some(color) = theme.theme.background {
            style.bg = Color::Rgb(color.r, color.g, color.b);
        }
    }

    if let Some(foreground) = chunk.foreground {
        style.fg = foreground;
    } else if let Some(theme) = chunk.theme {
        if let Some(color) = theme.theme.foreground {
            style.fg = Color::Rgb(color.r, color.g, color.b);
        }
    }

    if let Some(true) = chunk.underlined {
        style.modifier.contains(Modifier::UNDERLINED);
    }

    if let Some(true) = chunk.underlined {
        style.modifier.contains(Modifier::ITALIC);
    }

    style
}
