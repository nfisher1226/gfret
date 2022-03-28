use {
    clap_complete::{generate_to, shells},
    std::{env, io::Error},
};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };
    let mut cmd = build();
    let path = generate_to(shells::Bash, &mut cmd, "gfret", outdir.clone())?;
    println!(
        "cargo:warning=bash completion file is generated: {:?}",
        path
    );
    let path = generate_to(shells::Zsh, &mut cmd, "gfret", outdir.clone())?;
    println!("cargo:warning=zsh completion file is generated: {:?}", path);
    let path = generate_to(shells::Fish, &mut cmd, "gfret", outdir.clone())?;
    println!(
        "cargo:warning=fish completion file is generated: {:?}",
        path
    );
    let path = generate_to(shells::PowerShell, &mut cmd, "gfret", outdir)?;
    println!(
        "cargo:warning=pwsh completion file is generated: {:?}",
        path
    );
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
