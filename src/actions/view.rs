#[derive(Debug, PartialEq, Clone)]
pub enum ViewAction {
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
