use super::ViewAction;

#[derive(Debug, PartialEq, Clone)]
pub enum EditorAction {
    ListThemes,
    ListLanguages,
    NextView,
    PrevView,
    Open(Option<String>),
    SetTheme(String),
    View(ViewAction),
}
