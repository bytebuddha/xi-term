/// A widget is something that can be displayed on screen
mod view;
pub use self::view::View;
pub use self::view::ViewClient;

mod editor;
pub use self::editor::{ Editor, EditorResponse };

mod prompt;
pub use self::prompt::{ Prompt, PromptResponse, Message };

mod dev;
pub use self::dev::{ Dev, DevResponse };
