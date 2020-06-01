use super::XiTerm;
use core::ActionHandler;
use actions::{ Action, SystemAction };

impl ActionHandler<Action> for XiTerm {

    fn perform_action(&mut self, action: Action) {
        match action {
            Action::System(SystemAction::Quit) => self.exit = true,
            Action::Editor(action) => self.editor.perform_action(action)
        }
    }
}
