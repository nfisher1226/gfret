#![warn(clippy::all, clippy::pedantic)]
use std::cell::RefCell;

#[derive(Clone)]
pub struct File {
    saved_once: RefCell<bool>,
    saved_current: RefCell<bool>,
    filename: RefCell<String>,
}

impl File {
    pub fn init() -> File {
        File {
            saved_once: RefCell::new(false),
            saved_current: RefCell::new(false),
            filename: RefCell::new(String::from("")),
        }
    }

    pub fn saved(&self) -> bool {
       *self.saved_once.borrow()
   }

   pub fn set_saved(&self) {
       self.saved_once.swap(&RefCell::new(true));
   }

   pub fn filename(&self) -> Option<String> {
       if self.saved() {
           Some(self.filename.borrow().to_string())
       } else {
           None
       }
   }

   pub fn current(&self) -> bool {
       *self.saved_current.borrow()
   }

   pub fn set_current(&self) {
       self.saved_current.swap(&RefCell::new(true));
   }

   pub fn unset_current(&self) {
       self.saved_current.swap(&RefCell::new(false));
   }

    pub fn do_save(&self, filename: &str, document: &svg::Document) {
       match svg::save(&filename, document) {
           Ok(_) => println!("Output saved as {}.", filename),
           Err(e) => eprintln!("{}", e),
       };

       self.set_saved();
       self.set_current();
       self.filename.swap(&RefCell::new(String::from(filename)));
    }
}
