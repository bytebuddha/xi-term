/// A widget is something that can be displayed on screen
mod view;
pub use self::view::View;
pub use self::view::ViewClient;

mod editor;
pub use self::editor::Editor;

mod prompt;
pub use self::prompt::{ Prompt, PromptResponse, Message };
