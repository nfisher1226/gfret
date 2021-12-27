#![warn(clippy::all, clippy::pedantic)]
use fretboard_layout::{Config, Font, Units};
use rgba_simple::{Color, Primary, ReducedRGBA};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
use std::fs;

/// Returns an OS appropriate configuration directory path
pub fn get_config_dir() -> PathBuf {
    let mut configdir: PathBuf = gtk::glib::user_config_dir();
    let progname = env!("CARGO_PKG_NAME");
    configdir.push(progname);
    if !configdir.exists() {
        fs::create_dir(&configdir.to_str().unwrap()).unwrap_or_else(|e| eprintln!("{}", e));
    }
    configdir
}

/// Returns the path to config.toml
pub fn get_config_file() -> PathBuf {
    let mut file = get_config_dir();
    file.push("config.toml");
    file
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GfretConfig {
    /// Whether to use Metric (mm) or Imperial (in) measurements
    pub units: Units,
    /// An optional external program that can edit svg images
    pub external_program: Option<String>,
    /// The border which will appear around the rendering
    pub border: f64,
    /// The line weight for all of the elements in mm
    pub line_weight: f64,
    /// The color of the fret lines
    pub fretline_color: Color,
    /// The background color of the fretboard
    pub fretboard_color: Color,
    /// The color of the centerline
    pub centerline_color: Option<Color>,
    /// The font used for the specifications
    pub font: Option<Font>,
}

impl Default for GfretConfig {
    fn default() -> Self {
        GfretConfig {
            units: Units::default(),
            external_program: Some(String::from("inkscape")),
            border: 10.0,
            line_weight: 1.0,
            fretline_color: Color::Reduced(ReducedRGBA::white()),
            fretboard_color: Color::Reduced(ReducedRGBA::black()),
            centerline_color: Some(Color::Reduced(ReducedRGBA::blue())),
            font: Some(Font::default()),
        }
    }
}

impl GfretConfig {
    /// Saves Template struct as a .toml file
    pub fn save_to_file(&self, file: &Path) {
        let toml_string = toml::to_string(&self).expect("Could not encode TOML value");
        fs::write(file, toml_string).expect("Could not write to file!");
    }

    /// Deserializes config.toml into a `GfretConfig` struct
    pub fn from_file() -> Option<GfretConfig> {
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
        let config: GfretConfig = match toml::from_str(&config_file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };
        Some(config)
    }

    /// Maps a `GfretConfig` struct to a `fretboard_layout::Config` struct
    pub fn to_config(&self) -> Config {
        Config {
            units: self.units.clone(),
            border: self.border,
            line_weight: self.line_weight,
            fretline_color: self.fretline_color.clone(),
            fretboard_color: self.fretboard_color.clone(),
            centerline_color: self.centerline_color.clone(),
            font: self.font.clone(),
        }
    }
}
