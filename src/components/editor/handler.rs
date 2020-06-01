use crossterm::event::Event;

use super::Editor;
use core::EventHandler;

impl EventHandler for Editor {

    fn handle_event(&mut self, event: Event) {
        if let Some(view) = self.views.get_mut(&self.current_view) {
            view.handle_event(event)
        }
    }
}
