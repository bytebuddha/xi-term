use super::View;
use core::ActionHandler;
use actions::{ ViewAction, CursorAction };
use components::PromptResponse;

impl ActionHandler<ViewAction> for View {

    type Output = PromptResponse;

    fn perform_action(&mut self, action: ViewAction) -> PromptResponse {
        match action {
            ViewAction::SetLanguage(language) => self.client.set_lang(language),
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
            ViewAction::Save(file) => self.save(&file),
            ViewAction::Find(action) => self.client.find(action),
        }
        PromptResponse::Cancel
    }
}
