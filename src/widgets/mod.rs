mod editor;
pub use self::editor::EditorWidget;

pub mod view;
pub use self::view::{ ViewWidget };

pub mod prompt;
pub use self::prompt::PromptWidget;

mod dev;
pub use self::dev::DevWidget;
