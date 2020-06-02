use tui::layout::Rect;
use crossterm::event::Event;

pub trait EventHandler {

    type Output = ();

    fn handle_event(&mut self, event: Event) -> Self::Output;
}

pub trait ActionHandler<T> {

    type Output = ();

    fn perform_action(&mut self, action: T) -> Self::Output;
}

pub trait RenderCursor {

    fn render_cursor(&self, area: Rect) -> Option<(u16, u16)>;
}
