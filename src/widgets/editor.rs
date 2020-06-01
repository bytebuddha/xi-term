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
}

impl <'a>Widget for EditorWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(view) = self.editor.views.get(&self.editor.current_view) {
            ViewWidget::new(&self.editor, &view).render(area, buf)
        }
    }
}
