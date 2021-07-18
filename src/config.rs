#![warn(clippy::all, clippy::pedantic)]
use fretboard_layout::config::Config;
use serde::{Deserialize, Serialize};

use crate::CONFIGDIR;

use std::path::{ Path, PathBuf };
use std::{ fs, process };

/// Returns an OS appropriate configuration directory path
pub fn get_config_dir() -> PathBuf {
    let mut configdir: PathBuf = match xdg_basedir::get_config_home() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let progname = env!("CARGO_PKG_NAME");
    configdir.push(progname);
    if !configdir.exists() {
        fs::create_dir(&configdir.to_str().unwrap()).unwrap_or_else(|e| eprintln!("{}", e));
    }
    configdir
}

/// Returns the path to config.toml
pub fn get_config_file() -> PathBuf {
    let mut file = CONFIGDIR.clone();
    file.push("config.toml");
    file
}

/// Deserializes config.toml into a [Config] struct
pub fn from_file() -> Option<Config> {
    let config_file = get_config_file();
    let config_file = if config_file.exists() {
        match fs::read_to_string(config_file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        }
    } else {
        return None;
    };
    let config: Config = match toml::from_str(&config_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    };
    Some(config)
}

/// Saves Config struct as a .toml file
pub fn save_to_file(config: Config, file: &Path) {
    let toml_string = toml::to_string(&config).expect("Could not encode TOML value");
    fs::write(file, toml_string).expect("Could not write to file!");
}
