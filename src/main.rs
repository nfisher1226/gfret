#![warn(clippy::all, clippy::pedantic)]
// switch to this when it stabilizes
//#![feature(mutex_unlock)]
#![doc = include_str!("../README.md")]

use {fretboard_layout::Config, gui::file::File, std::sync::Mutex};
/// The cli
mod cli;
/// Handles getting the configuration data to and from disk
mod config;
mod convert;
pub mod error;
/// The Gtk user interface to gfret.
mod gui;
/// Persistent templates
mod template;
pub use convert::Convert;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIG: Mutex<Config> =
        Mutex::new(config::Config::from_file().unwrap_or_default().truncate());
    static ref FILE: Mutex<File> = Mutex::new(File::default());
}

fn main() {
    let matches = cli::opts::build().get_matches();
    if let Some(("cli", cli_matches)) = matches.subcommand() {
        cli::run(cli_matches);
    } else {
        crate::gui::run(matches.value_of("TEMPLATE"));
    }
}
