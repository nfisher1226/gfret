#![warn(clippy::all, clippy::pedantic)]
use std::error::Error;

#[derive(Clone, Default)]
pub struct File {
    saved_once: bool,
    saved_current: bool,
    filename: String,
}

impl File {
    pub fn saved(&self) -> bool {
        self.saved_once
    }

    pub fn filename(&self) -> Option<String> {
        if self.saved() {
            Some(self.filename.to_string())
        } else {
            None
        }
    }

    pub fn current(&self) -> bool {
        self.saved_current
    }

    pub fn unset_current(&mut self) {
        self.saved_current = false;
    }

    pub fn do_save(
        &mut self,
        filename: &str,
        document: &svg::Document,
    ) -> Result<(), Box<dyn Error>> {
        svg::save(&filename, document)?;
        self.saved_once = true;
        self.saved_current = true;
        self.filename = String::from(filename);
        Ok(())
    }
}
