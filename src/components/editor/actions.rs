use futures::Future;

use super::Editor;
use core::ActionHandler;
use actions::EditorAction;
use components::{ PromptResponse, Message };

impl ActionHandler<EditorAction> for Editor {

    type Output = PromptResponse;

    fn perform_action(&mut self, action: EditorAction) -> Self::Output {
        match action {
            EditorAction::View(action) => {
                if let Some(view) = self.views.get_mut(&self.current_view) {
                    view.perform_action(action);
                }
                PromptResponse::Continue
            },
            EditorAction::SetTheme(theme) => {
                 tokio::spawn(self.client.set_theme(&theme).map_err(|_| ()));
                 PromptResponse::Cancel
            },
            EditorAction::ListThemes => {
                PromptResponse::Message(Message::info(format!("{:?}", &self.themes)).title("Themes"))
            },
            EditorAction::ListLanguages => {
                PromptResponse::Message(Message::info(format!("{:?}", &self.languages)).title("Languages"))
            },
            EditorAction::Open(file) => {
                self.new_view(file);
                PromptResponse::Cancel
            },
            EditorAction::NextView => self.next_buffer(),
            EditorAction::PrevView => self.prev_buffer()
        }
    }
}
