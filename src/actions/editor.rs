use super::ViewAction;

#[derive(Debug, PartialEq, Clone)]
pub enum EditorAction {
    View(ViewAction),
}
