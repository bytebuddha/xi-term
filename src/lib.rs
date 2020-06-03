#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate clap;

extern crate failure;

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate futures;
extern crate indexmap;
extern crate crossterm;
extern crate tokio;
extern crate xdg;
extern crate xrl;
extern crate tui;

pub mod core;
pub mod ui;
pub mod widgets;
pub mod components;
pub mod actions;

pub use failure::Error;
use futures::{future, Future, Stream};
use xrl::spawn;

use core::ActionHandler;
use actions::EditorAction;
use ui::{XiTerm, XiTermServiceBuilder};

pub fn run() -> Result<(), Error> {
    let xi = clap_app!(
        xi =>
        (about: "The Xi Editor")
        (@arg core: -c --core +takes_value "Specify binary to use for the backend")
        (@arg logfile: -l --logfile +takes_value "Log file location")
        (@arg file: +required "File to edit"));

    let matches = xi.get_matches();
    if let Some(logfile) = matches.value_of("logfile") {
        core::configure_logs(logfile);
    }
    core::init_panic_handler();

    tokio::run(future::lazy(move || {
        info!("starting xi-core");
        let (tui_service_builder, core_events_rx) = XiTermServiceBuilder::new();
        let (client, core_stderr) = spawn(
            matches.value_of("core").unwrap_or("xi-core"),
            tui_service_builder,
        ).unwrap();

        info!("starting logging xi-core errors");
        tokio::spawn(
            core_stderr
                .for_each(|msg| {
                    error!("core: {}", msg);
                    Ok(())
                })
                .map_err(|_| ()),
        );

        tokio::spawn(future::lazy(move || {
            let conf_dir = core::get_config_directory();

            let client_clone = client.clone();
            client
                .client_started(conf_dir.as_deref(), None)
                .map_err(|e| error!("failed to send \"client_started\" {:?}", e))
                .and_then(move |_| {
                    info!("initializing the TUI");
                    let mut tui = XiTerm::new(client_clone, core_events_rx)
                        .expect("failed to initialize the TUI");
                    tui.editor.perform_action(EditorAction::SetTheme("base16-eighties.dark".into()));
                    tui.editor.new_view(
                        matches.value_of("file").map(ToString::to_string),
                    );
                    tui.map_err(|e| error!("TUI exited with an error: {:?}", e))
                })
        }));
        Ok(())
    }));
    Ok(())
}
