mod view;
pub use self::view::CurrentViewWidget;

mod logger;
pub use self::logger::LogWidget;

use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::style::{ Color, Style, Modifier };
use tui::layout::{ Layout, Direction, Constraint };
use tui::widgets::{Borders, Tabs, Block, Widget, StatefulWidget };

use components::Dev;

#[derive(Default)]
pub struct DevWidget;

impl StatefulWidget for DevWidget {

    type State = Dev;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
      buf.set_background(area, Color::DarkGray);
      let border_style = Style::default().bg(Color::DarkGray).fg(Color::Magenta);
      let title_style = Style::default().bg(Color::DarkGray).fg(Color::LightMagenta);
      let style = Style::default().bg(Color::DarkGray);
      let log_block = Block::default()
              .borders(Borders::ALL)
              .border_style(border_style)
              .title_style(title_style)
              .style(style)
              .title("Developer Tools");

      let chunks = Layout::default()
          .direction(Direction::Vertical)
          .margin(1)
          .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
          .split(area);

      Tabs::default()
          .block(log_block)
          .titles(&["Current View", "Logs"])
          .select(state.current_tab)
          .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::LightMagenta).modifier(Modifier::BOLD))
          .style(Style::default().bg(Color::DarkGray).fg(Color::Magenta))
          .divider("⌇")
          .render(chunks[0], buf);
      match state.current_tab {
          0 => CurrentViewWidget::new().render(chunks[1], buf),
          1 => LogWidget::new().render(chunks[1], buf),
          _ => unreachable!(),
      }
    }
}
