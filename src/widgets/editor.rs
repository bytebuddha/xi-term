use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;

use super::{ ViewWidget };
use crate::components::Editor;

pub struct EditorWidget<'a> {
    editor: &'a Editor
}

impl <'a>EditorWidget<'a> {

    pub fn new(editor: &'a Editor) -> EditorWidget<'a> {
        EditorWidget { editor }
    }

    pub fn calculate_view_rect(area: Rect) -> Rect {
        let x = EditorWidget::calculate_x_offset(area.y);
        let y = EditorWidget::calculate_y_offset(area.x);
        let height = EditorWidget::calculate_height_offset(area.height);
        let width = EditorWidget::calculate_width_offset(area.width);
        Rect { x, y, width, height }
    }

    pub fn calculate_height_offset(height: u16) -> u16 {
        height
    }

    pub fn calculate_y_offset(y: u16) -> u16 {
        y
    }

    pub fn calculate_width_offset(width: u16) -> u16 {
        width
    }

    pub fn calculate_x_offset(x: u16) -> u16 {
        x
    }
}

impl <'a>Widget for EditorWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(view) = self.editor.views.get(&self.editor.current_view) {
            let view_rect = EditorWidget::calculate_view_rect(area);
            ViewWidget::new(&self.editor, &view).render(view_rect, buf)
        }
    }
}
