mod view;
pub use self::view::CurrentViewWidget;

use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::style::Color;
use tui::widgets::{ Widget, StatefulWidget };

use components::{ Dev, Editor };

pub struct DevWidget<'a> {
    editor: &'a Editor
}

impl <'a>DevWidget<'a> {

    pub fn new(editor: &'a Editor) -> DevWidget<'a> {
        DevWidget { editor }
    }
}

impl <'a>StatefulWidget for DevWidget<'a> {

    type State = Dev;

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
      buf.set_background(area, Color::DarkGray);
      if let Some(view) = self.editor.views.get(&self.editor.current_view) {
          CurrentViewWidget::new(&view).render(area, buf)
      }
    }
}
