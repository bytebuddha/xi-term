mod terminal;
pub use self::terminal::{Terminal, TerminalEvent};

mod actions;
mod handler;
#[allow(clippy::module_inception)]
mod ui;
pub use self::ui::{CoreEvent, XiTerm, XiTermService};

mod builder;
pub use self::builder::XiTermServiceBuilder;
