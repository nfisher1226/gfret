use {
    adw::glib,
    serde::Deserialize,
    std::{collections::HashMap, fs, path::PathBuf},
};

#[derive(Default, Deserialize)]
pub struct Keys {
    pub keys: HashMap<String, String>,
}

fn get_key_file() -> PathBuf {
    let mut file = glib::user_config_dir();
    file.push(env!("CARGO_PKG_NAME"));
    file.push("keys.toml");
    file
}

impl Keys {
    #[must_use]
    pub fn get(&self, action: &str) -> &str {
        if let Some(key) = self.keys.get(action) {
            if adw::gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        match action {
            "open" => "<primary>O",
            "save" => "<primary>S",
            "save_as" => "<primary><Shift>S",
            "open_external" => "<primary>E",
            "preferences" => "<primary><Shift>P",
            "about" => "<primary>A",
            "quit" => "<primary>Q",
            _ => unreachable!(),
        }
    }

    pub fn from_file() -> Result<Self, crate::error::Error> {
        let keyfile = get_key_file();
        let keyfile = fs::read_to_string(keyfile)?;
        let keys: Self = toml::from_str(&keyfile)?;
        Ok(keys)
    }
}
