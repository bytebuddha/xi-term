use crossterm::event::{ Event, KeyCode };

use super::{ Dev, DevResponse };
use crate::core::EventHandler;

impl EventHandler for Dev {

    type Output = DevResponse;

    fn handle_event(&mut self, event: Event) -> Self::Output {
        debug!("handling input {:?}", event);
        match event {
            Event::Key(key) => {
                if let KeyCode::Char('q') = key.code {
                     return DevResponse::Close;
                }
            },
            _ => {}
        }
        DevResponse::Continue
    }
}
