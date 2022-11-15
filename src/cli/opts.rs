use {
    clap::{value_parser, Arg, ArgAction, Command},
    gettext::gettext,
};

/// The cli subcommand options
#[must_use]
pub fn build_cli() -> Command {
    Command::new("cli")
        .about(gettext("Output an svg without running the interface"))
        .long_about(
            gettext("Gfret can be run from the command line just as easily as it can from a graphical
interface, and sometimes more quickly if all of the desired specifications are
known in advance.")
        )
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .args([
            Arg::new("SCALE")
                .help(gettext("Scale length in mm"))
                .short('s')
                .long("scale")
                .value_parser(value_parser!(f64))
                .default_value("648"),
            Arg::new("MULTI")
                .help(gettext("Multiscale fretboard with <MULTI> as the treble scale"))
                .short('m')
                .long("multi")
                .value_parser(value_parser!(f64))
                .default_value("610"),
            Arg::new("LEFT")
                .help(gettext("Multiscale fretboard reversed (left handed)"))
                .short('l')
                .long("left")
                .action(ArgAction::SetTrue),
            Arg::new("PERPENDICULAR")
                .help(gettext("Which fret is perpendicular to the centerline"))
                .short('p')
                .long("perpendicular")
                .value_parser(value_parser!(f64))
                .default_value("8"),
            Arg::new("COUNT")
                .help(gettext("Total fret count"))
                .short('c')
                .long("count")
                .value_parser(value_parser!(u32))
                .default_value("24"),
            Arg::new("NUT")
                .help(gettext("Nut width"))
                .short('n')
                .long("nut")
                .value_parser(value_parser!(f64))
                .default_value("43"),
            Arg::new("BRIDGE")
                .help(gettext("Bridge Spacing"))
                .short('b')
                .long("bridge")
                .value_parser(value_parser!(f64))
                .default_value("56"),
            Arg::new("OUTPUT")
                .help(gettext("Name of the output file"))
                .default_value("output.svg"),
            Arg::new("EXTERN")
                .help(gettext("Open output file in external program"))
                .short('e')
                .long("external")
                .num_args(0..=1)
                .value_parser(value_parser!(String))
                .default_missing_value("inkscape"),
        ])
}

/// The main program options
#[must_use]
pub fn build() -> Command {
    Command::new("gfret")
        .about(gettext("Generates layout dimensions for a stringed instrument fretboard"))
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("FILE")
                .help(gettext("An svg image previously created by Gfret"))
                .num_args(1),
        )
        .subcommand(build_cli())
}
