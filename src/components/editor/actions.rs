use futures::Future;
use super::Editor;
use core::ActionHandler;
use actions::EditorAction;

impl ActionHandler<EditorAction> for Editor {

    fn perform_action(&mut self, action: EditorAction) {
        match action {
            EditorAction::View(action) => {
                if let Some(view) = self.views.get_mut(&self.current_view) {
                    view.perform_action(action);
                }
            },
            EditorAction::SetTheme(theme) => {
                 tokio::spawn(self.client.set_theme(&theme).map_err(|_| ()));
            }
        }
    }
}
