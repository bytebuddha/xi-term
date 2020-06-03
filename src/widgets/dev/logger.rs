use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::{ Widget, Borders, Block };
use tui::style::{ Style, Color };

pub struct LogWidget {

}

impl LogWidget {

    pub fn new() -> LogWidget {
        LogWidget {

        }
    }
}

impl Widget for LogWidget {

    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan))
                .style(Style::default().bg(Color::DarkGray))
                .render(area, buf);
    }
}
