mod imp;

use adw::{
    gtk::{self, gio, glib::{self, Object}},
    prelude::*,
    subclass::{
        prelude::*,
        window::AdwWindowImpl,
        preferences_window::PreferencesWindowImpl
    },
};

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
        @extends adw::PreferencesWindow, adw::Window, gtk::Window,
            gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl PreferencesWindow {
    #[must_use]
     pub fn new() -> Self {
        let obj: Self = Object::new(&[]).expect("Cannot create preferences window");
        obj
     }
}