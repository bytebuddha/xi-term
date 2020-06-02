use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;
use tui::style::{ Color, Style };

use components::View;

pub struct Gutter<'a> {
    view: &'a View,
    start: Option<u64>
}

impl <'a>Gutter<'a> {

    pub fn new(view: &'a View) -> Gutter<'a> {
        Gutter { view, start: None }
    }

    pub fn start(mut self, start: u64) -> Gutter<'a> {
        self.start = Some(start);
        self
    }

    fn get_style(&self) -> Style {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    }
}

impl <'a>Widget for Gutter<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_background(area, Color::DarkGray);
        for (dex, line_number) in self.view.cache.lines().iter()
                .skip(self.view.window.start() as usize)
                .take(area.height as usize)
                .map(|item| {
                    item.line_num.map(|item|format!("{}", item))
                })
                .enumerate()
                .collect::<Vec<(usize, Option<String>)>>() {
                if let Some(line_no) = line_number {
                    if line_no.len() == 1 {
                        buf.set_stringn(3, area.y + dex as u16, line_no, 3, self.get_style());
                    } else if line_no.len() == 2 {
                        buf.set_stringn(2, area.y + dex as u16, line_no, 3, self.get_style());
                    } else if line_no.len() == 3 {
                        buf.set_stringn(1, area.y + dex as u16, line_no, 3, self.get_style());
                    } else {
                        buf.set_stringn(1, area.y + dex as u16, &line_no[line_no.len() - 3..], 3, self.get_style());
                    }
                }
        }
    }
}
