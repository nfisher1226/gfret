#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]

use {fretboard_layout::Config, std::sync::Mutex};
/// The cli
mod backend;
mod cli;
/// Handles getting the configuration data to and from disk
mod config;
/// The Gtk user interface to gfret.
mod gui;
/// Persistent templates
mod template;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(
        config::GfretConfig::from_file()
            .unwrap_or_default()
            .to_config()
    );
}

fn main() {
    let matches = cli::build().get_matches();
    if let Some(("cli", cli_matches)) = matches.subcommand() {
        backend::run(&cli_matches);
    } else {
        crate::gui::run(matches.value_of("TEMPLATE"));
    }
}
