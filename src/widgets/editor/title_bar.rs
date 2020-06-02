use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;
use tui::style::Color;
use tui::style::Style;

use components::Editor;

pub struct TitleBar<'a> {
    editor: &'a Editor
}

impl <'a>TitleBar<'a> {

    pub fn new(editor: &'a Editor) -> TitleBar<'a> {
        TitleBar { editor }
    }

    fn get_background_style(&self) -> Color {
        Color::DarkGray
    }

    fn get_tab_style(&self) -> Style {
        Style::default().bg(Color::DarkGray).fg(Color::Gray)
    }
}

impl <'a>Widget for TitleBar<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_background(area, self.get_background_style());

        let tabs = "•".repeat(self.editor.views.len());
        let x = (area.x + area.width - tabs.len() as u16) / 2;
        buf.set_string(x, area.y, &tabs, self.get_tab_style());
        let view_index = if let Some(index) = self.editor.views.get_index_of(&self.editor.current_view) {
            x + index as u16
        } else {
            x
        };
        buf.get_mut(view_index, area.y).set_fg(Color::White);

        if let Some(view) = self.editor.views.get(&self.editor.current_view) {
            let x = area.x + area.width - 2;
            let y = area.y;
            if view.pristine {
                buf.get_mut(x, y).set_fg(Color::Green).set_symbol("☑");
            } else {
                buf.get_mut(x, y).set_fg(Color::Red).set_symbol("✗");
            }
        }
    }
}
