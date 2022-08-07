mod imp;

use {
    adw::{prelude::*, subclass::prelude::*},
    gtk::{
        gdk::Display,
        gio,
        glib::{self, clone, Object},
        prelude::*,
        subclass::prelude::*,
        CssProvider, StyleContext,
    },
};

glib::wrapper! {
    pub struct GfretWindow(ObjectSubclass<imp::GfretWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, adw::Window,
            gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
            gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root,
            gtk::ShortcutManager;
}

impl GfretWindow {
    #[must_use]
    pub fn new(app: &adw::Application) -> Self {
        let obj: Self = Object::new(&[("application", app)]).expect("Cannot create GfretWindow");
        obj
    }
}
