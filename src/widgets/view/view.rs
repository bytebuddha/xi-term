use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::style::Color;
use tui::widgets::Widget;

use super::LineWidget;
use crate::components::Editor;
use crate::components::View;

pub struct ViewWidget<'a, 'b> {
    editor: &'a Editor,
    view: &'b View
}

impl <'a, 'b>ViewWidget<'a, 'b> {

    pub fn new(editor: &'a Editor, view: &'b View) -> ViewWidget<'a, 'b> {
        ViewWidget { editor, view }
    }
}

impl <'a, 'b>Widget for ViewWidget<'a, 'b> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(theme) = &self.editor.theme {
            if let Some(color) = theme.theme.background {
                buf.set_background(area, Color::Rgb(color.r, color.g, color.b));
            }
        }
        let lines = self.view.cache.lines().iter()
            .skip(self.view.window.start() as usize)
            .take(self.view.window.size() as usize);

        for (line_index, line) in lines.enumerate() {
            let start_y = area.y + line_index as u16;
            let start_x = area.x;
            if start_y < area.height + area.y && start_x < area.width + area.x {
                let line_rect = Rect { x: start_x, y: start_y, width: area.width, height: 1};
                LineWidget::new(&self.editor, &self.view).line(line).render(line_rect, buf);
            }
        }

        let line_count = self.view.cache.lines().len() as u16;
        let win_size = self.view.window.size();
        if win_size > line_count {
            for num in line_count..win_size {
                let start_y = area.y + num as u16;
                let start_x = area.x;
                if start_y < area.height + area.y && start_x < area.width + area.x {
                    let line_rect = Rect { x: start_x, y: start_y, width: area.width, height: 1};
                    LineWidget::new(&self.editor, &self.view).render(line_rect, buf);
                }
            }
        }
    }
}
