pub mod opts;

use {
    clap::ArgMatches,
    fretboard_layout::{Handedness, Specs, Variant},
    std::process,
};

/// Runs the cli
pub fn run(matches: &ArgMatches) {
    let scale: f64 = match matches.get_one::<f64>("SCALE") {
        Some(c) => *c,
        None => {
            process::exit(1);
        }
    };
    let count = match matches.get_one("COUNT") {
        Some(c) => *c,
        None => {
            process::exit(1);
        }
    };
    let variant = if let Some(c) = matches.get_one("MULTI") {
        let scale = *c;
        let handedness = if matches.get_flag("LEFT") {
            Handedness::Left
        } else {
            Handedness::Right
        };
        let pfret = match matches.get_one("PERPENDICULAR") {
            Some(c) => *c,
            None => {
                process::exit(1);
            }
        };
        Variant::Multiscale {
            scale,
            handedness,
            pfret,
        }
    } else {
        Variant::Monoscale
    };
    let nut = match matches.get_one("NUT") {
        Some(c) => *c,
        None => {
            process::exit(1);
        }
    };
    let bridge: f64 = match matches.get_one("BRIDGE") {
        Some(c) => c + 6.0,
        None => {
            process::exit(1);
        }
    };
    let specs = Specs::init(scale, count, variant, nut, bridge);
    let doc = specs.create_document(None);
    let output = matches.get_one::<String>("OUTPUT").map(|x| x.to_string()).unwrap_or("-".to_string());
    if output == "-" {
        println!("{doc}");
    } else {
        match svg::save(&output, &doc) {
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
