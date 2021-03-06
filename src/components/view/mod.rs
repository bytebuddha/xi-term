mod client;

#[allow(clippy::module_inception)]
mod view;
mod window;
mod cursor;

pub use self::client::Client as ViewClient;
pub use self::view::View;

mod handler;
mod actions;
