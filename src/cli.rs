use clap::{Arg, Command};

pub fn build_cli() -> Command<'static> {
    Command::new("cli")
        .about("Output an svg without running the interface")
        .long_about(
"Gfret can be run from the commmand line just as easily as it can from a graphical
interface, and sometimes more quickly if all of the desired specifications are
known in advance."
        )
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("SCALE")
                .help("Scale length in mm")
                .short('s')
                .long("scale")
                .takes_value(true)
                .default_value("648"),
        )
        .arg(
            Arg::new("MULTI")
                .help("Multiscale fretboard with <MULTI> as the treble scale")
                .short('m')
                .long("multi")
                .takes_value(true)
                .default_value("610"),
        )
        .arg(
            Arg::new("LEFT")
                .help("Multiscale fretboard reversed (left handed)")
                .short('l')
                .long("left")
                .takes_value(false),
        )
        .arg(
            Arg::new("PERPENDICULAR")
                .help("Which fret is perpendicular to the centerline")
                .short('p')
                .long("perpendicular")
                .takes_value(true)
                .default_value("8"),
        )
        .arg(
            Arg::new("COUNT")
                .help("Total fret count")
                .short('c')
                .long("count")
                .takes_value(true)
                .default_value("24"),
        )
        .arg(
            Arg::new("NUT")
                .help("Nut width")
                .short('n')
                .long("nut")
                .takes_value(true)
                .default_value("43"),
        )
        .arg(
            Arg::new("BRIDGE")
                .help("Bridge Spacing")
                .short('b')
                .long("bridge")
                .takes_value(true)
                .default_value("56"),
        )
        .arg(
            Arg::new("OUTPUT")
                .help("Name of the output file")
                .takes_value(true)
                .default_value("output.svg"),
        )
        .arg(
            Arg::new("EXTERN")
                .help("Open output file in external program")
                .short('e')
                .long("external")
                .takes_value(true)
                .default_value("inkscape"),
        )
}

pub fn build() -> Command<'static> {
    Command::new("gfret")
        .about("Generates layout dimensions for a stringed instrument fretboard")
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("TEMPLATE")
                .help("A valid Gfret template in toml format")
                .short('t')
                .long("template")
                .takes_value(true),
        )
        .subcommand(build_cli())
}
