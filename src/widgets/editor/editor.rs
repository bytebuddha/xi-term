use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;

use super::{ TitleBar, Gutter };
use widgets::{ ViewWidget };
use crate::components::Editor;

pub struct EditorWidget<'a> {
    editor: &'a Editor
}

impl <'a>EditorWidget<'a> {

    pub fn new(editor: &'a Editor) -> EditorWidget<'a> {
        EditorWidget { editor }
    }

    pub fn calculate_view_rect(display_title_bar: bool, display_gutter: bool, area: Rect) -> Rect {
        let x = EditorWidget::calculate_x_offset(display_gutter, area.y);
        let y = EditorWidget::calculate_y_offset(display_title_bar, area.x);
        let height = EditorWidget::calculate_height_offset(display_title_bar, area.height);
        let width = EditorWidget::calculate_width_offset(display_gutter, area.width);
        Rect { x, y, width, height }
    }

    pub fn calculate_title_bar_rect(area: Rect) -> Rect {
        Rect { x: area.x, y: area.y, width: area.width, height: 1 }
    }

    pub fn calculate_height_offset(display_title_bar: bool, height: u16) -> u16 {
        if display_title_bar {
            height - 1
        } else {
            height
        }
    }

    pub fn calculate_y_offset(display_title_bar: bool, y: u16) -> u16 {
        if display_title_bar {
            y + 1
        } else {
            y
        }
    }

    pub fn calculate_width_offset(display_gutter: bool, width: u16) -> u16 {
        if display_gutter {
            width - 4
        } else {
            width
        }
    }

    pub fn calculate_x_offset(display_gutter: bool, x: u16) -> u16 {
        if display_gutter {
            x + 4
        } else {
            x
        }
    }

    pub fn calculate_gutter_rect(&self, display_title_bar: bool, rect: Rect) -> Rect {
        if display_title_bar {
            Rect { x: 0, y: 1, width: 4, height: rect.height - 1 }
        } else {
            Rect { x: 0, y: 0, width: 4, height: rect.height }
        }
    }
}

impl <'a>Widget for EditorWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(view) = self.editor.views.get(&self.editor.current_view) {
            let view_rect = EditorWidget::calculate_view_rect(self.editor.display_title_bar, self.editor.display_gutter, area);
            ViewWidget::new(&self.editor, &view).render(view_rect, buf);
            if self.editor.display_title_bar {
                let title_bar_rect = EditorWidget::calculate_title_bar_rect(area);
                TitleBar::new(&self.editor).render(title_bar_rect, buf);
            }

            if self.editor.display_gutter {
                let gutter_rect = self.calculate_gutter_rect(self.editor.display_title_bar, area);
                Gutter::new(&view).start(view.cache.before()).render(gutter_rect, buf);
            }
        }
    }
}
