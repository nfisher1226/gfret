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
//! * [Building](#building)
//! * [Roadmap](#roadmap)
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
//! gfret-cli
//!
//! Output an svg without running the interface
//!
//! USAGE:
//!    gfret cli [OPTIONS] [OUTPUT]
//!
//! ARGS:
//!    <OUTPUT>    Name of the output file [default: output.svg]
//!
//! OPTIONS:
//!    -b, --bridge <BRIDGE>
//!            Bridge spacing [default: 56]
//!
//!    -c, --count <COUNT>
//!            Total fret count [default: 24]
//!
//!    -e, --external <EXTERN>
//!            Open output file in external program [default: inkscape]
//!
//!    -h, --help
//!            Print help information
//!
//!    -l, --left
//!            Multiscale fretboard reversed (left handed)
//!
//!    -m, --multi <MULTI>
//!            Creates a multiscale fretboard with <MULTI> as the treble scale. [default: 610]
//!
//!    -n, --nut <NUT>
//!            Nut width [default: 43]
//!
//!    -o, --output <OUTPUT>
//!            Name of the output file [default: output.svg]
//!
//!    -p, --perpendicular <PERPENDICULAR>
//!            Set which fret is perpendicular to the centerline [default: 8]
//!    -s, --scale <SCALE>
//!            Scale length in mm [default: 648]
//! ```
//! ## Keybindings
//! | Key | Action |
//! | --- | --- |
//! | Ctrl/S | save file |
//! | Ctrl/Shift/S | save file as |
//! | Ctrl/E | open with an external program |
//! | Ctrl/O | load a template from file |
//! | Ctrl/Shift/P | open the preferences dialog |
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
//! ## must match either "Metric" or "Imperial"
//! units = "Metric"
//! ## the command line to run
//! external_program = "inkscape"
//! ## the size of the border around the image
//! border = 10.0
//! ## how thick the lines are
//! line_weight = 1.0
//!
//! [fretline_color]
//! ## Colors must be "Hex", "Reduced" or "RGBA"
//! ## "Reduced" will take a whole number between 0 and 255 for each channel,
//! ## while "RGBA" takes a decimal between 0.0 and 1.0.
//!
//! ## "Hex" will have a <color> field and an <alpha> field
//! ## ColorType = "Hex"
//! ## color = "#00ff00"
//! ## alpha = 1.0
//! ColorType = "Reduced"
//! red = <u8>
//! green = <u8>
//! blue = <u8>
//! alpha = <u8>
//!
//! [fretboard_color]
//! ColorType = "Reduced"
//! red = 36
//! green = 31
//! blue = 49
//! alpha = 255
//!
//! [centerline_color]
//! ColorType = "Reduced"
//! red = 0
//! green = 0
//! blue = 255
//! alpha = 255
//!
//! ## Fonts take a family and weight (style), but size is ignored
//! [font]
//! family = "Sans"
//! weight = "Normal"
//! ```
//! > Note: The graphical interface has a preferences dialog and will take care
//! > of maintaining the preferences file for you. There will be no need to edit
//! > this file by hand in normal use.
//! ## Building
//! You will need a Rust toolchain installed, including cargo. Gtk+3x is also
//! required. To build the program, run ```cargo build --release``` to build a
//! release binary in target/release.

//! Alternatively, you can use the included Makefile to build and install the
//! program, adjusting the installation path with the PREFIX and DESTDIR variables.

//! ## Roadmap
//! * For the gui, it would be nice to save state and allow loading specs from and saving
//! to templates. **partial implementation 4/7/21** | **completed 5/5/21**
//! * Port to Gtk4 **completed 12/21**
//! * Support changing from metric to imperial measurements **completed 12/21**
//! * Support left handed multiscale fretboards **completed 12/21**

use clap::{App, Arg};
use std::path::PathBuf;
/// Takes the command line arguments and launches either the gui or the cli
mod backend;
/// Handles getting the configuration data to and from disk
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
    let app = App::new("gfret")
        .about("Generates layout dimensions for a stringed instrument fretboard")
        .author("The JenG3nie <jeang3nie@hitchhiker-linux.org>")
        .arg(Arg::new("TEMPLATE")
            .help("A valid Gfret template in toml format")
            .short('t')
            .long("template")
            .takes_value(true)
        )
        .subcommand(App::new("cli")
            .about("Output an svg without running the interface")
            .arg(Arg::new("SCALE")
                .help("Scale length in mm")
                .short('s')
                .long("scale")
                .takes_value(true)
                .default_value("648")
            )
            .arg(Arg::new("MULTI")
                .help("Multiscale fretboard with <MULTI> as the treble scale")
                .short('m')
                .long("multi")
                .takes_value(true)
                .default_value("610")
            )
            .arg(Arg::new("LEFT")
                .help("Multiscale fretboard reversed (left handed)")
                .short('l')
                .long("left")
                .takes_value(false)
            )
            .arg(Arg::new("PERPENDICULAR")
                .help("Which fret is perpendicular to the centerline")
                .short('p')
                .long("perpendicular")
                .takes_value(true)
                .default_value("8")
            )
            .arg(Arg::new("COUNT")
                .help("Total fret count")
                .short('c')
                .long("count")
                .takes_value(true)
                .default_value("24")
            )
            .arg(Arg::new("NUT")
                .help("Nut width")
                .short('n')
                .long("nut")
                .takes_value(true)
                .default_value("43")
            )
            .arg(Arg::new("BRIDGE")
                .help("Bridge Spacing")
                .short('b')
                .long("bridge")
                .takes_value(true)
                .default_value("56")
            )
            .arg(Arg::new("OUTPUT")
                .help("Name of the output file")
                .takes_value(true)
                .default_value("output.svg")
            )
            .arg(Arg::new("EXTERN")
                .help("Open output file in external program")
                .short('e')
                .long("external")
                .takes_value(true)
                .default_value("inkscape")
            ),
        );
    let matches = app.get_matches();

    backend::run(&matches);
}
