#![warn(clippy::all, clippy::pedantic)]
use {
    crate::config,
    fretboard_layout::Handedness,
    serde::{Deserialize, Serialize},
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

/// The Template struct defines a data set that can be loaded to return to some
/// previous state in the program or to save a common design to return to later
#[derive(Deserialize, Debug, Serialize)]
pub struct Template {
    pub scale: f64,
    pub count: u32,
    pub scale_treble: Option<f64>,
    pub handedness: Option<Handedness>,
    pub nut: f64,
    pub bridge: f64,
    pub pfret: Option<f64>,
}

impl Template {
    /// Takes a filename as an argument and returns either a populated Template
    /// struct, or else None.
    pub fn load_from_file(file: PathBuf) -> Option<Template> {
        let file = if file.exists() {
            match fs::read_to_string(file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    return None;
                }
            }
        } else {
            return None;
        };
        let template: Template = match toml::from_str(&file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };
        Some(template)
    }
    /// Saves Template struct as a .toml file
    pub fn save_to_file(&self, file: &Path) -> Result<(), crate::error::Error> {
        let toml_string = toml::to_string(&self)?;
        let mut file = file.to_path_buf();
        file.set_extension("toml");
        fs::write(file, toml_string)?;
        Ok(())
    }
    /// Saves the program state on exit
    pub fn save_statefile(&self) {
        let mut statefile = config::get_config_dir();
        statefile.push("state.toml");
        if let Err(e) = self.save_to_file(&statefile) {
            eprintln!("Error saving statefile: {e}");
        }
    }
}
