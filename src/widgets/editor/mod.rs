mod title_bar;
pub use self::title_bar::TitleBar;

mod gutter;
pub use self::gutter::Gutter;

use serde_json::Value;
use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::{ Widget, StatefulWidget };

use core::consts::{ DEFAULT_DISPLAY_GUTTER, DEFAULT_DISPLAY_TITLE_BAR };
use widgets::{ ViewWidget };
use crate::components::Editor;

#[derive(Default)]
pub struct EditorWidget;

impl EditorWidget {

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

impl StatefulWidget for EditorWidget {

    type State = Editor;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if let Value::Bool(true) = state.config.get_default("display_title_bar", DEFAULT_DISPLAY_TITLE_BAR) {
            let title_bar_rect = EditorWidget::calculate_title_bar_rect(area);
            TitleBar::new(&state).render(title_bar_rect, buf);
        }
        if let Some(view) = state.views.get_mut(&state.current_view) {
            let display_gutter = if let Value::Bool(true) = state.config.get_default("display_gutter", DEFAULT_DISPLAY_GUTTER) {
                true
            } else {
                false
            };
            let display_title = if let Value::Bool(true) = state.config.get_default("display_title_bar", DEFAULT_DISPLAY_TITLE_BAR) {
                true
            } else {
                false
            };
            let view_rect = EditorWidget::calculate_view_rect(display_title, display_gutter, area);
            ViewWidget::new(&state.styles).theme(state.theme.as_ref()).render(view_rect, buf, view);
            view.rect = Some(view_rect);

            if let Value::Bool(true) = state.config.get_default("display_gutter", DEFAULT_DISPLAY_GUTTER) {
                let title_bar = if let Value::Bool(true) = state.config.get_default("display_title_bar", DEFAULT_DISPLAY_TITLE_BAR) {
                    true
                } else {
                    false
                };
                let gutter_rect = self.calculate_gutter_rect(title_bar, area);
                Gutter::new(&view).start(view.cache.before()).theme(state.theme.as_ref()).render(gutter_rect, buf);
            }
        }
    }
}
