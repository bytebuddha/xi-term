use tui::style::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub text: String,
    pub title: Option<String>,
    pub border_fg: Color,
    pub border_bg: Color,
    pub title_fg: Color,
    pub title_bg: Color
}

impl Message {

    pub fn error(text: String) -> Message {
        Message {
            text,
            title: Some("Error".into()),
            border_fg: Color::Red,
            border_bg: Color::DarkGray,
            title_fg: Color::LightYellow,
            title_bg: Color::DarkGray
        }
    }

    pub fn info(text: String) -> Message {
        Message {
            text,
            title: None,
            border_fg: Color::Blue,
            border_bg: Color::DarkGray,
            title_fg: Color::Cyan,
            title_bg: Color::DarkGray
        }
    }

    pub fn title<S: Into<String>>(mut self, s: S) -> Message {
        self.title = Some(s.into());
        self
    }
}
