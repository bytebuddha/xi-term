mod view;
pub use self::view::{ ViewAction, CursorAction };

mod editor;
pub use self::editor::EditorAction;

mod reactor;
pub use self::reactor::ActionReactor;

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Editor(EditorAction),
    System(SystemAction)
}

#[derive(Debug, PartialEq, Clone)]
pub enum SystemAction {
    Quit
}
