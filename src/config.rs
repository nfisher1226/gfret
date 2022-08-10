#![warn(clippy::all, clippy::pedantic)]
use {
    crate::error::Error,
    fretboard_layout::{
        Font, Primary,
        PrimaryColor::{Black, Blue, White},
        Units, RGBA,
    },
    adw::gtk::glib,
    serde::{Deserialize, Serialize},
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

/// Returns an OS appropriate configuration directory path
pub fn get_config_dir() -> PathBuf {
    let mut configdir: PathBuf = glib::user_config_dir();
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
pub struct Config {
    /// Whether to use Metric (mm) or Imperial (in) measurements
    pub units: Units,
    /// An optional external program that can edit svg images
    pub external_program: Option<String>,
    /// The border which will appear around the rendering
    pub border: f64,
    /// The line weight for all of the elements in mm
    pub line_weight: f64,
    /// The color of the fret lines
    pub fretline_color: RGBA<u8>,
    /// The background color of the fretboard
    pub fretboard_color: RGBA<u8>,
    /// The color of the centerline
    pub centerline_color: Option<RGBA<u8>>,
    /// The font used for the specifications
    pub font: Option<Font>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            units: Units::default(),
            external_program: Some(String::from("inkscape")),
            border: 10.0,
            line_weight: 1.0,
            fretline_color: RGBA::primary(White),
            fretboard_color: RGBA::primary(Black),
            centerline_color: Some(RGBA::primary(Blue)),
            font: Some(Font::default()),
        }
    }
}

impl Config {
    /// Saves Config struct as a .toml file
    pub fn save_to_file(&self, file: &Path) -> Result<(), Error> {
        let toml_string = toml::to_string(&self)?;
        fs::write(file, toml_string)?;
        Ok(())
    }

    /// Deserializes config.toml into a `GfretConfig` struct
    pub fn from_file() -> Result<Self, Error> {
        let config_file = get_config_file();
        let config_file = fs::read_to_string(config_file)?;
        let config: Self = toml::from_str(&config_file)?;
        Ok(config)
    }

    /// Maps a `GfretConfig` struct to a `fretboard_layout::Config` struct
    pub fn truncate(&self) -> fretboard_layout::Config {
        fretboard_layout::Config {
            units: self.units,
            border: self.border,
            line_weight: self.line_weight,
            fretline_color: self.fretline_color,
            fretboard_color: self.fretboard_color,
            centerline_color: self.centerline_color,
            font: self.font.clone(),
        }
    }
}
