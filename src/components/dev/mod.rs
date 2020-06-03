mod handler;

#[derive(Debug, PartialEq, Clone)]
pub enum DevResponse {
    Continue,
    Close
}

pub struct Dev {
    pub current_tab: usize
}

impl Default for Dev {

    fn default() -> Dev {
        Dev {
            current_tab: 0
        }
    }
}
