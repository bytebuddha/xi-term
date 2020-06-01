mod logs;
pub use self::logs::configure_logs;

use xdg::BaseDirectories;

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
