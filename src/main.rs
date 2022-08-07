fn main() {
    let matches = gfret::cli::opts::build().get_matches();
    match matches.subcommand() {
        Some(("cli", cli_matches)) => gfret::cli::run(cli_matches),
        //_ => gfret::gui::run(matches.value_of("TEMPLATE")),
        _ => gfret::run_gui(),
    }
}
