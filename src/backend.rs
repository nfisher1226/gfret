#![warn(clippy::all, clippy::pedantic)]
use clap::ArgMatches;
use fretboard_layout::{Handedness, Specs, Variant};

use std::process;
use std::process::Command;

/// When this function runs it either launches the gui or calls run(&specs) to
/// generate output, based on the command line arguments given to the program
pub fn run(matches: &ArgMatches) {
    if let Some(("cli", cli_matches)) = matches.subcommand() {
        let scale: f64 = match cli_matches.value_of_t("SCALE") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        let count = match cli_matches.value_of_t("COUNT") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        let variant = if cli_matches.occurrences_of("MULTI") > 0 {
            let scale = match cli_matches.value_of_t("MULTI") {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            };
            let hand = if cli_matches.occurrences_of("LEFT") > 0 {
                Handedness::Left
            } else {
                Handedness::Right
            };
            Variant::Multiscale(scale, hand)
        } else {
            Variant::Monoscale
        };
        let nut = match cli_matches.value_of_t("NUT") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        let bridge = match cli_matches.value_of_t::<f64>("BRIDGE") {
            Ok(c) => c + 6.0,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        let pfret = match cli_matches.value_of_t("PERPENDICULAR") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        let specs = Specs::init(scale, count, variant, nut, bridge, pfret);
        let doc = specs.create_document(None);
        let output = cli_matches.value_of("OUTPUT").unwrap().to_string();
        if output == "-" {
            println!("{}", doc);
        } else {
            match svg::save(&output, &doc) {
                Ok(_) => println!("Output saved as {}.", output),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            };
            if cli_matches.occurrences_of("EXTERN") > 0 {
                if let Some(cmd) = cli_matches.value_of("EXTERN") {
                    match Command::new(&cmd).args(&[&output]).spawn() {
                        Ok(_) => (),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
        }
    } else {
        crate::gui::run(matches.value_of("TEMPLATE"));
    }
}
