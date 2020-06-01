use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;
use tui::style::{ Color, Modifier };

pub struct Chunk<'a> {
    text: &'a str,
    background: Option<Color>,
    foreground: Option<Color>,
    italic: Option<bool>,
    underlined: Option<bool>
}

impl <'a>Chunk<'a> {

    pub fn new(text: &'a str) -> Chunk<'a> {
        Chunk {
            text,
            background: None,
            foreground: None,
            italic: None,
            underlined: None
        }
    }

    pub fn foreground(mut self, c: Option<Color>) -> Chunk<'a> {
        self.foreground = c;
        self
    }

    pub fn background(mut self, c: Option<Color>) -> Chunk<'a> {
        self.background = c;
        self
    }

    pub fn underlined(mut self, c: Option<bool>) -> Chunk<'a> {
        self.underlined = c;
        self
    }

    pub fn italic(mut self, c: Option<bool>) -> Chunk<'a> {
        self.italic = c;
        self
    }
}

impl <'a>Widget for Chunk<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut style = tui::style::Style::default();

        if let Some(background) = self.background {
            if self.text.len() == 0 {
                buf.set_background(area, background);
                return;
            }
            style.bg = background;
        }

        if let Some(foreground) = self.foreground {
            style.fg = foreground;
        }

        if let Some(true) = self.underlined {
            style.modifier.contains(Modifier::UNDERLINED);
        }

        if let Some(true) = self.underlined {
            style.modifier.contains(Modifier::ITALIC);
        }

        buf.set_stringn(
            area.x,
            area.y,
            &self.text,
            area.width as usize,
            style
        );
    }

}
