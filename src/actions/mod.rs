mod view;
pub use self::view::{ ViewAction, CursorAction, FindAction };

mod editor;
pub use self::editor::EditorAction;

mod reactor;
pub use self::reactor::ActionReactor;

mod parse;
pub use self::parse::parse_action;

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Ui(UiAction),
    ShellCommand(Vec<String>),
    Editor(EditorAction),
    System(SystemAction)
}

#[derive(Debug, PartialEq, Clone)]
pub enum SystemAction {
    Quit
}

#[derive(Debug, PartialEq, Clone)]
pub enum UiAction {
    ShowPrompt,
    HidePrompt,
    ToggleTitleBar,
    ToggleLineNumbers
}
