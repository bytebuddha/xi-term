use super::View;
use core::ActionHandler;
use actions::{ ViewAction, CursorAction };

impl ActionHandler<ViewAction> for View {

    fn perform_action(&mut self, action: ViewAction) {
        match action {
            ViewAction::Cursor(CursorAction::Up) => self.client.up(),
            ViewAction::Cursor(CursorAction::Down) => self.client.down(),
            ViewAction::Cursor(CursorAction::Left) => self.client.left(),
            ViewAction::Cursor(CursorAction::Right) => self.client.right(),
            ViewAction::Cursor(CursorAction::PageUp) => self.client.page_up(),
            ViewAction::Cursor(CursorAction::PageDown) => self.client.page_down(),
            ViewAction::Cursor(CursorAction::Backspace) => self.client.backspace(),
            ViewAction::Cursor(CursorAction::Delete) => self.client.delete(),
            ViewAction::Cursor(CursorAction::Home) => self.client.home(),
            ViewAction::Cursor(CursorAction::End) => self.client.end(),
        }
    }
}
