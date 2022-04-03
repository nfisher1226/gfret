#[cfg(any(feature = "completions", feature = "png-icons", feature = "manpage"))]
use std::{ env, ffi::OsString };

#[cfg(feature = "completions")]
use clap_complete::{generate_to, shells};

#[cfg(feature = "manpage")]
use clap_mangen::Man;

#[cfg(any(feature = "manpage", feature = "png-icons"))]
use std::path::PathBuf;

#[cfg(feature = "png-icons")]
use {
    std::fs,
    tiny_skia::Transform,
    usvg::{ FitTo, Options, Tree },
};

use std::error::Error;

#[cfg(any(feature = "completions", feature = "manpage"))]
include!("src/cli.rs");

#[cfg(feature = "completions")]
fn completions(outdir: &OsString) -> Result<(), Box<dyn Error>> {
    let mut cmd = build();
    let _path = generate_to(shells::Bash, &mut cmd, "gfret", outdir.clone())?;
    let _path = generate_to(shells::Zsh, &mut cmd, "gfret", outdir.clone())?;
    let _path = generate_to(shells::Fish, &mut cmd, "gfret", outdir.clone())?;
    let _path = generate_to(shells::PowerShell, &mut cmd, "gfret", outdir.clone())?;
    Ok(())
}

#[cfg(feature = "manpage")]
fn manpage(outdir: &OsString) -> Result<(), Box<dyn Error>> {
    let cmd = build();
    let file: PathBuf = [outdir.to_str().unwrap(), "gfret.1"].iter().collect();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer)?;
    std::fs::write(file, buffer)?;
    let cmd = build_cli();
    let file: PathBuf = [outdir.to_str().unwrap(), "gfret-cli.1"].iter().collect();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer)?;
    std::fs::write(file, buffer)?;
    Ok(())
}

#[cfg(feature = "png-icons")]
fn png(tree: &Tree, size: u32, outdir: &OsString) -> Result<(), Box<dyn Error>> {
    let fit = FitTo::Size(size, size);
    let transform = Transform::from_scale(1.0, 1.0);
    let mut pixmap = match tiny_skia::Pixmap::new(size, size) {
        Some(p) => p,
        None => return Err(String::from("Error creating png").into()),
    };
    resvg::render(&tree, fit, transform, pixmap.as_mut());
    let file = format!("gfret{}x{}.png", size, size);
    let mut outfile = PathBuf::from(outdir);
    outfile.push(&file);
    pixmap.save_png(outfile)?;
    Ok(())
}

#[cfg(feature = "png-icons")]
fn iconvert(outdir: &OsString) -> Result<(), Box<dyn Error>> {
    let infile: PathBuf = ["data", "gfret.svg"].iter().collect();
    let data = fs::read(&infile)?;
    let tree = Tree::from_data(&data, &Options::default().to_ref())?;
    for size in [256, 128, 64, 48, 32] {
        png(&tree, size, outdir)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(any(feature = "completions", feature = "png-icons", feature = "manpage"))]
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    #[cfg(feature = "completions")]
    completions(&outdir)?;

    #[cfg(feature = "manpage")]
    manpage(&outdir)?;

    #[cfg(feature = "png-icons")]
    iconvert(&outdir)?;

    #[cfg(any(feature = "png-icons", feature = "completions", feature = "manpage"))]
    println!("cargo:warning=generated files are in: {:?}", outdir);

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
