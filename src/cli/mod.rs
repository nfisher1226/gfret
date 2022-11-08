pub mod opts;

use {
    clap::ArgMatches,
    fretboard_layout::{Handedness, Specs, Variant},
    std::process,
};

/// Runs the cli
/// # Panics
/// Because all of the values retrieved using `unwrap()` have a default value
/// set, this function should never actually panic.
pub fn run(matches: &ArgMatches) {
    let scale = *matches.get_one("SCALE").unwrap();
    let count = *matches.get_one("COUNT").unwrap();
    let variant = if let Some(s) = matches.get_one("MULTI") {
        let handedness = if matches.get_flag("LEFT") {
            Handedness::Left
        } else {
            Handedness::Right
        };
        let pfret = *matches.get_one("PERPENDICULAR").unwrap();
        Variant::Multiscale {
            scale: *s,
            handedness,
            pfret,
        }
    } else {
        Variant::Monoscale
    };
    let nut = *matches.get_one("NUT").unwrap();
    let bridge = matches.get_one("BRIDGE").unwrap() + 6.0;
    let specs = Specs::init(scale, count, variant, nut, bridge);
    let doc = specs.create_document(None);
    let output = matches
        .get_one::<String>("OUTPUT")
        .map_or("-", std::string::String::as_str);
    if output == "-" {
        println!("{doc}");
    } else {
        match svg::save(output, &doc) {
            Ok(_) => println!("Output saved as {output}."),
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        if let Some(cmd) = matches.get_one::<String>("EXTERN") {
            match process::Command::new(cmd).args([&output]).spawn() {
                Ok(_) => (),
                Err(e) => eprintln!("{e}"),
            }
        }
    }
}
