use crossterm::event::{ Event, KeyModifiers, KeyCode };

use super::XiTerm;
use components::PromptResponse;
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
            let event = match event {
                Event::Key(event) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                        if let KeyCode::Char('c') = event.code {
                            return;
                        }
                    }
                    Event::Key(event)
                },
                event => event
            };

            if let Some(prompt) = &mut self.prompt {
                match prompt.handle_event(event) {
                    PromptResponse::Continue => {},
                    PromptResponse::Cancel => {
                        self.prompt = None;
                    },
                    PromptResponse::Message(msg) => prompt.set_message(msg),
                    PromptResponse::Action(action) => self.perform_action(action)
                }
            } else {
                self.editor.handle_event(event);
            }
        }
    }
}
