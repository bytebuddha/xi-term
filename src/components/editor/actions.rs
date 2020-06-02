use futures::Future;

use super::Editor;
use core::ActionHandler;
use actions::EditorAction;
use components::{ EditorResponse, Message };

impl ActionHandler<EditorAction> for Editor {

    type Output = EditorResponse;

    fn perform_action(&mut self, action: EditorAction) -> Self::Output {
        match action {
            EditorAction::View(action) => {
                if let Some(view) = self.views.get_mut(&self.current_view) {
                    From::from(view.perform_action(action))
                } else {
                    EditorResponse::Continue
                }
            },
            EditorAction::SetTheme(theme) => {
                 tokio::spawn(self.client.set_theme(&theme).map_err(|_| ()));
                 EditorResponse::Cancel
            },
            EditorAction::ListThemes => {
                EditorResponse::Message(Message::info(format!("{:?}", &self.themes)).title("Themes"))
            },
            EditorAction::ListLanguages => {
                EditorResponse::Message(Message::info(format!("{:?}", &self.languages)).title("Languages"))
            },
            EditorAction::Open(file) => {
                self.new_view(file);
                EditorResponse::Cancel
            },
            EditorAction::NextView => self.next_buffer(),
            EditorAction::PrevView => self.prev_buffer()
        }
    }
}
