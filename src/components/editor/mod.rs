#[allow(clippy::module_inception)]
mod editor;
pub use self::editor::Editor;

mod configuration;
pub use self::configuration::Configuration;

mod handler;
mod actions;

use super::Message;
use components::PromptResponse;
use actions::Action;

#[derive(Debug, PartialEq, Clone)]
pub enum EditorResponse {
    Message(Message),
    Action(Action),
    Continue,
    Cancel
}

impl From<PromptResponse> for EditorResponse {
    fn from(f: PromptResponse) -> EditorResponse {
        match f {
            PromptResponse::Continue => EditorResponse::Continue,
            PromptResponse::Cancel => EditorResponse::Cancel,
            PromptResponse::Action(action) => EditorResponse::Action(action),
            PromptResponse::Message(msg) => EditorResponse::Message(msg),
        }
    }
}
