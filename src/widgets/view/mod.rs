mod line;
pub use self::line::LineWidget;

mod chunk;
pub use self::chunk::Chunk;

use std::collections::HashMap;

use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::style::Color;
use tui::widgets::StatefulWidget;
use tui::widgets::Widget;
use xrl::{ Style, ThemeChanged };

use crate::components::View;

pub struct ViewWidget<'a, 'b> {
    styles: &'a HashMap<u64, Style>,
    theme: Option<&'b ThemeChanged>
}

impl <'a, 'b>ViewWidget<'a, 'b> {

    pub fn new(styles: &'a HashMap<u64, Style>) -> ViewWidget<'a, 'b> {
        ViewWidget { styles, theme: None }
    }

    pub fn theme(mut self, theme: Option<&'b ThemeChanged>) -> ViewWidget<'a, 'b> {
        self.theme = theme;
        self
    }
}

impl <'a, 'b>StatefulWidget for ViewWidget<'a, 'b> {

    type State = View;

    fn render(self, area: Rect, buf: &mut Buffer, view: &mut Self::State) {
        if let Some(theme) = &self.theme {
            if let Some(color) = theme.theme.background {
                buf.set_background(area, Color::Rgb(color.r, color.g, color.b));
            }
        }
        let lines = view.cache.lines().iter()
            .skip(view.window.start() as usize)
            .take(view.window.size() as usize);

        for (line_index, line) in lines.enumerate() {
            let start_y = area.y + line_index as u16;
            let start_x = area.x + 1;
            if start_y < area.height + area.y && start_x < area.width + area.x {
                let line_rect = Rect { x: start_x, y: start_y, width: area.width, height: 1};
                LineWidget::new(&self.styles, &view).line(line).theme(self.theme).render(line_rect, buf);
            }
        }

        let line_count = view.cache.lines().len() as u16;
        let win_size = view.window.size();
        if win_size > line_count {
            for num in line_count..win_size {
                let start_y = area.y + num as u16;
                let start_x = area.x + 1;
                if start_y < area.height + area.y && start_x < area.width + area.x {
                    let line_rect = Rect { x: start_x, y: start_y, width: area.width, height: 1};
                    LineWidget::new(&self.styles, &view).theme(self.theme).render(line_rect, buf);
                }
            }
        }
    }
}
