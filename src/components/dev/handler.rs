use crossterm::event::{ Event, KeyCode };

use super::{ Dev, DevResponse };
use crate::core::EventHandler;

impl EventHandler for Dev {

    type Output = DevResponse;

    fn handle_event(&mut self, event: Event) -> Self::Output {
        debug!("handling input {:?}", event);
        if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Char('q') => {
                        return DevResponse::Close;
                    },
                    _ => {}
                }
        }
        DevResponse::Continue
    }
}
