use crossterm::event::{ Event, KeyCode };

use super::{ Prompt, PromptResponse };
use core::EventHandler;

impl EventHandler for Prompt {

    type Output = PromptResponse;

    fn handle_event(&mut self, event: Event) -> PromptResponse {
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Enter => self.finalize(),
                    KeyCode::Backspace => self.back(),
                    KeyCode::Delete => self.delete(),
                    KeyCode::Left => self.left(),
                    KeyCode::Right => self.right(),
                    KeyCode::Char(c) => self.new_key(c),
                    key => {
                        warn!("Unhandleded keycode: {:?}", key);
                        PromptResponse::Continue
                    }
                }
            },
            event => {
                warn!("Unhandleded Key Event: {:?}", event);
                PromptResponse::Continue
            }
        }
    }
}
