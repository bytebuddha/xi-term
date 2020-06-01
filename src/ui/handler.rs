use crossterm::event::{ Event, KeyModifiers, KeyCode };

use super::XiTerm;
use crate::core::{ EventHandler, ActionHandler };

impl EventHandler for XiTerm {
    fn handle_event(&mut self, event: Event) {
        debug!("handling input {:?}", event);
        if let Some(actions) = self.actions.event_to_action(&event) {
            for action in actions {
                self.perform_action(action.clone());
                return;
            }
        } else {
            match event {
                Event::Key(event) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                        if let KeyCode::Char('c') = event.code {
                            return;
                        }
                    }
                    self.editor.handle_event(Event::Key(event))
                },
                event => {
                    self.editor.handle_event(event)
                }
            }
        }
    }
}
