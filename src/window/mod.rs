mod imp;

use {
    adw::{prelude::*, subclass::prelude::*},
    gtk::{
        gdk::Display,
        gio,
        glib::{self, clone, Object},
        prelude::*,
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
        obj.connect_signals();
        obj
    }

    fn connect_signals(&self) {
        self.imp().variant_box.connect_changed(clone!(@strong self as win => move |bx| {
            let set = bx.active_id() == Some(glib::GString::from("multiscale"));
            win.imp().handedness_box.set_visible(set);
            win.imp().scale_multi.set_visible(set);
            win.imp().scale_multi_fine.set_visible(set);
            win.imp().pfret_label.set_visible(set);
            win.imp().perpendicular_fret.set_visible(set);
        }));
    }
}
