use tui::layout::Rect;
use tui::style::{ Style, Color };
use tui::buffer::Buffer;
use tui::widgets::Widget;

pub struct InputWidget<'a> {
    chars: &'a str
}

impl <'a>InputWidget<'a> {
    pub fn new(chars: &'a str) -> InputWidget<'a> {
        InputWidget { chars }
    }

    fn get_prompt_style(&self) -> Style {
        let mut style = Style::default();
        style.fg = Color::White;
        style.bg = Color::DarkGray;
        style
    }
}

impl <'a>Widget for InputWidget<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_background(area, Color::DarkGray);
        let cell = buf.get_mut(area.x, area.y);
        cell.style = self.get_prompt_style();
        cell.symbol = "»".into();
        let cell = buf.get_mut(area.x + 1, area.y);
        cell.style = self.get_prompt_style();
        cell.symbol = " ".into();
        buf.set_string(area.x + 2, area.y, self.chars,  self.get_prompt_style());

        for letter in area.x as usize + 2 + self.chars.len()..area.width as usize - (self.chars.len() as usize + 2) {
            let cell = buf.get_mut(letter as u16, area.y);
            cell.symbol = " ".into();
        }
    }
}
