use adw::gtk::prelude::*;

fn main() {
    let matches = gfret::cli::opts::build().get_matches();
    match matches.subcommand() {
        Some(("cli", cli_matches)) => gfret::cli::run(cli_matches),
        _ => {
            let app = gfret::Application::new();
            app.run();
        }
    }
}
