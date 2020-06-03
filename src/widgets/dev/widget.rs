use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::layout::{ Layout, Direction, Constraint };
use tui::widgets::{Borders, Tabs, Block, Widget, StatefulWidget };

use super::LogWidget;
use components::Dev;

pub struct DevWidget;

impl DevWidget {

    pub fn new() -> DevWidget {
        DevWidget
    }
}

impl StatefulWidget for DevWidget {

    type State = Dev;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
      let log_block = Block::default().title("Developer Tools").borders(Borders::ALL);

      let chunks = Layout::default()
          .direction(Direction::Vertical)
          .margin(1)
          .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
          .split(area);

      Tabs::default()
          .block(log_block)
          .titles(&["Logs"])
          .divider("DOT")
          .render(chunks[0], buf);
      let inner = match state.current_tab {
          0 => LogWidget::new(),
          _ => unreachable!(),
      };
      inner.render(chunks[1], buf);
    }
}
