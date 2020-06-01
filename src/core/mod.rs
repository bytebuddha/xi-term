mod logs;
pub use self::logs::configure_logs;

mod style;
pub use self::style::u32_to_color;

use xdg::BaseDirectories;
use crossterm::event::Event;

pub fn init_panic_handler() {
    use std::panic;

    panic::set_hook(Box::new(|err| {
        error!("Fatal Crash: {:?}", err);
    }));
}

pub fn get_config_directory() -> Option<String> {
    BaseDirectories::with_prefix("xi").ok()
        .and_then(|dirs| Some(dirs.get_config_home().to_string_lossy().into_owned()))
}

pub trait EventHandler {

    fn handle_event(&mut self, event: Event);
}

pub trait ActionHandler<T> {

    fn perform_action(&mut self, action: T);
}
