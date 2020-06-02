#[derive(Debug, PartialEq, Clone)]
pub enum ViewAction {
    Save(String),
    SetLanguage(String),
    Cursor(CursorAction),
    Find(FindAction)
}

#[derive(Debug, PartialEq, Clone)]
pub enum CursorAction {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Backspace,
    Delete,
    Home,
    End,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FindAction {
    Query(String, bool, bool, bool),
    Next(bool, bool),
    Previous(bool, bool)
}
