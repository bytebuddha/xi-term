use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::{ Borders, Widget, Block };
use tui::style::{ Color, Style };

use components::Message;

pub struct MessageBox<'a> {
    msg: &'a Message,
    wrapped_lines: Option<String>
}

impl <'a>MessageBox<'a> {

    pub fn new(msg: &'a Message) -> MessageBox<'a> {
        MessageBox { msg, wrapped_lines: None }
    }

    pub fn wrapped(mut self, text: Vec<String>) -> MessageBox<'a> {
        self.wrapped_lines = Some(text.join("\n"));
        self
    }


    fn get_border_style(&self) -> Style {
        Style::default()
            .fg(self.msg.border_fg)
            .bg(self.msg.border_bg)
    }

    fn get_title_style(&self) -> Style {
        Style::default()
            .fg(self.msg.title_fg)
            .bg(self.msg.title_bg)
    }
}

impl <'a>Widget for MessageBox<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_background(area, Color::DarkGray);
        let mut block = Block::default()
            .borders(Borders::ALL)
            .title_style(self.get_title_style())
            .border_style(self.get_border_style());
        if let Some(header) = &self.msg.title {
            block = block.title(&header);
        }
        block.render(area, buf);
        let data_rect = block.inner(area);
        buf.set_background(data_rect, Color::DarkGray);
        let string = if let Some(data) = self.wrapped_lines {
            data
        } else {
            self.msg.text.clone()
        };
        for (line_index, line) in string.lines().enumerate() {
            let style = Style::default().bg(Color::DarkGray).fg(Color::White);
            buf.set_stringn(data_rect.x, data_rect.y + line_index as u16, line, line.len(), style);
            let remains = (data_rect.x + data_rect.width) - line.len() as u16;
            let y = data_rect.y + line_index as u16;
            let x = data_rect.x + line.len() as u16;
            buf.set_stringn(x, y, &" ".repeat(remains as usize), remains as usize - 1, style);
        }
    }
}
