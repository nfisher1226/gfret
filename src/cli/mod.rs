pub mod opts;

use {
    clap::ArgMatches,
    fretboard_layout::{Handedness, Specs, Variant},
    std::process,
};

/// Runs the cli
pub fn run(matches: &ArgMatches) {
    let scale: f64 = match matches.value_of_t("SCALE") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    let count = match matches.value_of_t("COUNT") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    let variant = if matches.occurrences_of("MULTI") > 0 {
        let scale = match matches.value_of_t("MULTI") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        let handedness = if matches.occurrences_of("LEFT") > 0 {
            Handedness::Left
        } else {
            Handedness::Right
        };
        let pfret = match matches.value_of_t("PERPENDICULAR") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{e}");
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
    let nut = match matches.value_of_t("NUT") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    let bridge = match matches.value_of_t::<f64>("BRIDGE") {
        Ok(c) => c + 6.0,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    let specs = Specs::init(scale, count, variant, nut, bridge);
    let doc = specs.create_document(None);
    let output = matches.value_of("OUTPUT").unwrap_or("-").to_string();
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
        if matches.occurrences_of("EXTERN") > 0 {
            if let Some(cmd) = matches.value_of("EXTERN") {
                match process::Command::new(cmd).args([&output]).spawn() {
                    Ok(_) => (),
                    Err(e) => eprintln!("{e}"),
                }
            }
        }
    }
}
