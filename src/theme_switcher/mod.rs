mod imp;

use adw::gtk::{
    self,
    glib::{self, Object},
};

glib::wrapper! {
    pub struct ThemeSwitcher(ObjectSubclass<imp::ThemeSwitcher>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ThemeSwitcher {
    fn default() -> Self {
        Object::new(&[])
    }
}

impl ThemeSwitcher {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
