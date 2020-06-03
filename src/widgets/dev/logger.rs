use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;

pub struct LogWidget {

}

impl LogWidget {

    pub fn new() -> LogWidget {
        LogWidget {

        }
    }
}

impl Widget for LogWidget {

    fn render(self, _area: Rect, _buf: &mut Buffer) {

    }
}
