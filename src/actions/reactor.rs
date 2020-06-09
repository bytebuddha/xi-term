use crossterm::event::{ KeyCode, KeyEvent, Event, KeyModifiers };

use std::collections::HashMap;

use actions::{ Action, SystemAction, UiAction };

#[derive(Debug)]
pub enum ReactorError {
    AlreadyBound
}

pub struct ActionReactor {
    data: HashMap<Event, Vec<Action>>
}

impl ActionReactor {

    pub fn new() -> ActionReactor {
        ActionReactor {
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, ev: Event, action: Vec<Action>) -> Result<(), ReactorError> {
        if self.data.contains_key(&ev) {
            return Err(ReactorError::AlreadyBound);
        }
        self.data.insert(ev, action);
        Ok(())
    }

    pub fn remove(&mut self, ev: Event) -> Result<(), ReactorError> {
        self.data.remove(&ev);
        Ok(())
    }

    pub fn event_to_action(&self, ev: &Event) -> Option<Vec<Action>> {
        self.data.get(ev).map(|item| item.to_vec())
    }
}

impl Default for ActionReactor {
    fn default() -> ActionReactor {
        let mut reactor = ActionReactor::new();
        let modifiers = KeyModifiers::CONTROL;
        let event = Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers });
        reactor.insert(event, vec![Action::System(SystemAction::Quit)]).unwrap();

        let event = Event::Key(KeyEvent { code: KeyCode::Char('p'), modifiers });
        reactor.insert(event, vec![Action::Ui(UiAction::ShowPrompt)]).unwrap();

        let event = Event::Key(KeyEvent { code: KeyCode::F(12), modifiers: KeyModifiers::empty() });
        reactor.insert(event, vec![Action::Ui(UiAction::ToggleDebugWidget)]).unwrap();
        reactor
    }
}
