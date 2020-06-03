use crossterm::event::Event;

use super::XiTerm;
use components::{ PromptResponse, DevResponse };
use crate::core::{ EventHandler, ActionHandler };

impl EventHandler for XiTerm {
    fn handle_event(&mut self, event: Event) {
        debug!("handling input {:?}", event);
        if let Some(actions) = self.actions.event_to_action(&event) {
            for action in actions {
                self.perform_action(action);
            }
        } else {
            let mut close_dev = false;
            if let Some(dev) = &mut self.dev {
                match dev.handle_event(event) {
                    DevResponse::Close => close_dev = true,
                    DevResponse::Continue => {},
                }
            }
            if close_dev {
                self.dev = None;
                return;
            }
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
