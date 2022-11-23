use {
    adw::gtk::prelude::*,
    gettext::{bind_textdomain_codeset, TextDomain, TextDomainError},
    std::error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = TextDomain::new("gfret")
        .push("/usr/local/share")
        .init() {
        match e {
            TextDomainError::TextDomainCallFailed(_) |
            TextDomainError::BindTextDomainCallFailed(_) |
            TextDomainError::BindTextDomainCodesetCallFailed(_) => return Err(e.into()),
            _ => {},
        }
    }
    bind_textdomain_codeset("gfret", "UTF-8")?;
    let matches = gfret::cli::opts::build().get_matches();
    match matches.subcommand() {
        Some(("cli", cli_matches)) => gfret::cli::run(cli_matches),
        _ => {
            let app = gfret::Application::new();
            app.run();
        }
    }
    Ok(())
}
