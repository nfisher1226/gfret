#![warn(clippy::all, clippy::pedantic)]
//! Contents
//! ========
//! * [Introduction](#introduction)
//! * [Usage](#usage)
//!   * [Cli](#running-the-command-line-interface)
//!   * [Gui](#running-the-gui)
//!     * [Keybindings](#keybindings)
//!   * [Templates](#templates)
//! * [Configuration](#configuration)
//!
//! ## Introduction
//! Gfret renders an svg image template of a fretboard for a stringed instrument.
//! It has a Gtk interface as well as a command line interface and can produce
//! templates for instruments ranging from a piccolo mandolin to an upright bass.
//! Multiscale designs are also supported. Currently, all measurements are
//! expressed in metric units only.
//! ## Running the gui
//! Calling the program by invoking ```gfret``` without any arguments will run
//! the Gtk interface. Additionally, a .desktop file and icon are included and
//! will be installed if the program is installed using the included
//! ```Makefile```, and can be used for launching the program from desktop menus
//! or creating shortcuts.
//! ## Running the command line interface
//! ```Bash
//! USAGE:
//!    gfret cli [OPTIONS] [SCALE]
//!
//! ARGS:
//!    <SCALE>    Scale length in mm. [default: 648]
//!
//! FLAGS:
//!    -h, --help       Prints help information
//!    -V, --version    Prints version information
//!
//! OPTIONS:
//!    -b, --bridge <BRIDGE>                  Bridge spacing [default: 56]
//!    -c, --count <COUNT>                    Total fret count [default: 24]
//!    -e, --external <EXTERN>                Open output file in external program [default: inkscape]
//!    -m, --multi <MULTI>
//!            Creates a multiscale fretboard with <MULTI> as the treble scale. [default: 610]
//!
//!    -n, --nut <NUT>                        Nut width [default: 43]
//!    -o, --output <OUTPUT>                  Name of the output file [default: output.svg]
//!    -p, --perpendicular <PERPENDICULAR>
//!            Set which fret is perpendicular to the centerline [default: 8]
//! ```
//! ## Keybindings
//! | Key | Action |
//! | --- | --- |
//! | Ctrl/S | save file |
//! | Ctrl/Shift/S | save file as |
//! | Ctrl/E | open with an external program |
//! | Ctrl/O | load a template from file |
//! | Ctrl/P | open the preferences dialog |
//! | Ctrl/Q | quit the program |
//! ## Templates
//! Along with the svg output, Gfret will save the specifications used to
//! generate the rendering in a Toml file with it's name corresponding to the
//! name of the svg file. These templates can be loaded later, either as an
//! argument when invoking the program, in which case the output will be
//! immediately generated, or else loaded from the Gui interface for further
//! editing. This is useful for sharing a common scale among multiple designs to
//! use as a starting point.
//! ## Configuration
//! On Unix systems the default configuration directory is ```~/.config/gfret```.
//! Gfret will maintain a configuration file here in [Toml](https://github.com/toml-lang/toml)
//! format, with the following fields:
//! ```Toml
//! external_program = String
//! border = f64
//! line_weight = f64
//! fretline_color = rgba String
//! fretboard_color = rgba String
//! draw_centerline = bool
//! centerline_color = rgba String
//! print_specs = bool
//! font = String
//! background_color = rgba String
//! ```

use clap::{crate_version, load_yaml, App};
use std::path::PathBuf;
mod backend;
mod config;
/// The Gtk user interface to gfret.
mod gui;
/// Persistent templates
mod template;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIGDIR: PathBuf = config::get_config_dir();
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).version(crate_version!()).get_matches();
    backend::run(&matches);
}
