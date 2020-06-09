mod view;
pub use self::view::{ ViewAction, CursorAction, FindAction };

mod editor;
pub use self::editor::EditorAction;

mod reactor;
pub use self::reactor::ActionReactor;

mod parse;
pub use self::parse::parse_action;

mod event;
pub use self::event::parse_event;

use crossterm::event::Event;
use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Ui(UiAction),
    ShellCommand(Vec<String>),
    Editor(EditorAction),
    System(SystemAction),
    Settings(SettingsAction)
}

#[derive(Debug, PartialEq, Clone)]
pub enum SettingsAction {
    Config(ConfigAction)
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConfigAction {
    Set(String, Value),
    UnSet(String),
    Get(String),
    Bind(Event, Vec<Action>),
    UnBind(Event)
}

#[derive(Debug, PartialEq, Clone)]
pub enum SystemAction {
    Quit
}

#[derive(Debug, PartialEq, Clone)]
pub enum UiAction {
    ShowPrompt,
    HidePrompt,
    ToggleDebugWidget,
    ShowDebugWidget,
    HideDebugWidget
}
