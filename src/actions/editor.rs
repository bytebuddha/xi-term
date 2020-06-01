use super::ViewAction;

#[derive(Debug, PartialEq, Clone)]
pub enum EditorAction {
    SetTheme(String),
    View(ViewAction),
}
