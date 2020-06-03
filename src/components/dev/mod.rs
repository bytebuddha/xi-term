mod dev;
pub use self::dev::Dev;

mod handler;

#[derive(Debug, PartialEq, Clone)]
pub enum DevResponse {
    Continue,
    Close
}
