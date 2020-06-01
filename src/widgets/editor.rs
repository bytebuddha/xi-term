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
        Rect { x: area.x, y: area.y, width: area.width, height: area.height }
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
