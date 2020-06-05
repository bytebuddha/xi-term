mod handler;

#[derive(Debug, PartialEq, Clone)]
pub enum DevResponse {
    Continue,
    Close
}

#[derive(Default)]
pub struct Dev {

}
