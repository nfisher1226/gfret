#![warn(clippy::all, clippy::pedantic)]

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
        self.saved_current = false
    }

    pub fn do_save(&mut self, filename: &str, document: &svg::Document) {
        match svg::save(&filename, document) {
            Ok(_) => {
                println!("Output saved as {}.", filename);
                self.saved_once = true;
                self.saved_current = true;
                self.filename = String::from(filename);
            },
            Err(e) => eprintln!("{}", e),
        };

    }
}
