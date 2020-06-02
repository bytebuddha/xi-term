#[derive(Debug, PartialEq, Clone)]
pub enum ViewAction {
    Save(String),
    SetLanguage(String),
    Cursor(CursorAction)
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
