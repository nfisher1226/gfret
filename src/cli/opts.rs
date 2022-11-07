use clap::{Arg, ArgAction, Command, value_parser};

/// The cli subcommand options
#[must_use]
pub fn build_cli() -> Command {
    Command::new("cli")
        .about("Output an svg without running the interface")
        .long_about(
            "Gfret can be run from the command line just as easily as it can from a graphical
interface, and sometimes more quickly if all of the desired specifications are
known in advance.",
        )
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("SCALE")
                .help("Scale length in mm")
                .short('s')
                .long("scale")
                .value_parser(value_parser!(f64))
                .default_value("648"),
        )
        .arg(
            Arg::new("MULTI")
                .help("Multiscale fretboard with <MULTI> as the treble scale")
                .short('m')
                .long("multi")
                .value_parser(value_parser!(f64))
                .default_value("610"),
        )
        .arg(
            Arg::new("LEFT")
                .help("Multiscale fretboard reversed (left handed)")
                .short('l')
                .long("left")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("PERPENDICULAR")
                .help("Which fret is perpendicular to the centerline")
                .short('p')
                .long("perpendicular")
                .value_parser(value_parser!(f64))
                .default_value("8"),
        )
        .arg(
            Arg::new("COUNT")
                .help("Total fret count")
                .short('c')
                .long("count")
                .value_parser(value_parser!(u32))
                .default_value("24"),
        )
        .arg(
            Arg::new("NUT")
                .help("Nut width")
                .short('n')
                .long("nut")
                .value_parser(value_parser!(f64))
                .default_value("43"),
        )
        .arg(
            Arg::new("BRIDGE")
                .help("Bridge Spacing")
                .short('b')
                .long("bridge")
                .value_parser(value_parser!(f64))
                .default_value("56"),
        )
        .arg(
            Arg::new("OUTPUT")
                .help("Name of the output file")
                .default_value("output.svg"),
        )
        .arg(
            Arg::new("EXTERN")
                .help("Open output file in external program")
                .short('e')
                .long("external")
                .num_args(0..=1)
                .value_parser(value_parser!(String))
                .default_missing_value("inkscape"),
        )
}

/// The main program options
#[must_use]
pub fn build() -> Command {
    Command::new("gfret")
        .about("Generates layout dimensions for a stringed instrument fretboard")
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("FILE")
                .help("An svg image previously created by Gfret")
                .num_args(1),
        )
        .subcommand(build_cli())
}
