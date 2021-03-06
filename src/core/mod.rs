pub mod consts;

mod logs;
pub use self::logs::configure_logs;

mod style;
pub use self::style::u32_to_color;

mod traits;
pub use self::traits::{ RenderCursor, ActionHandler, EventHandler };

use xdg::BaseDirectories;

pub fn init_panic_handler() {
    use std::panic;

    panic::set_hook(Box::new(|err| {
        error!("Fatal Crash: {:?}", err);
    }));
}

pub fn get_config_directory() -> Option<String> {
    BaseDirectories::with_prefix("xi").ok()
        .map(|dirs| dirs.get_config_home().to_string_lossy().into_owned())
}
