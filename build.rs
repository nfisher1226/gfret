#[cfg(any(feature = "completions", feature = "png-icons"))]
use std::{ env, ffi::OsString };

#[cfg(feature = "completions")]
use clap_complete::{generate_to, shells};

#[cfg(feature = "png-icons")]
use {
    std::{
        fs,
        path::PathBuf,
    },
    tiny_skia::Transform,
    usvg::{ FitTo, Options, Tree },
};

use std::error::Error;

#[cfg(feature = "completions")]
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

#[cfg(feature = "png-icons")]
fn png(tree: &Tree, size: u32, outdir: &OsString) -> Result<(), Box<dyn Error>>{
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
    #[cfg(any(feature = "completions", feature = "png-icons"))]
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    #[cfg(feature = "completions")]
    completions(&outdir)?;

    #[cfg(feature = "png-icons")]
    iconvert(&outdir)?;

    #[cfg(all(feature = "png-icons", feature = "completions"))]
    println!("cargo:warning=completions and icons generated in: {:?}", outdir);

    #[cfg(all(feature = "png-icons", not(feature = "completions")))]
    println!("cargo:warning=icons generated in: {:?}", outdir);

    #[cfg(all(feature = "completions", not(feature = "png-icons")))]
    println!("cargo:warning=completions generated in: {:?}", outdir);

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
