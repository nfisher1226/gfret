mod imp;

use {
    adw::{prelude::*, subclass::prelude::*},
    crate::{
        config::{self, Config},
        template::Template,
        Convert, CONFIG, FILE,
    },
    fretboard_layout::{Handedness, Specs, Units, Variant},
    gtk::{
        gdk::Display,
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream},
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
            let set = bx.active() == Some(1) || bx.active() == Some(2);
            win.toggle_multi(set);
            win.draw_preview();
        }));
        self.imp().scale.connect_value_changed(clone!(@weak self as win => move |scl| {
            win.draw_preview();
        }));
        self.imp().scale_multi.connect_value_changed(clone!(@weak self as win => move |scl| {
            win.draw_preview();
        }));
        self.imp().nut_width.connect_value_changed(clone!(@weak self as win => move |scl| {
            win.draw_preview();
        }));
        self.imp().bridge_spacing.connect_value_changed(clone!(@weak self as win => move |scl| {
            win.draw_preview();
        }));
        self.imp().perpendicular_fret.connect_value_changed(clone!(@weak self as win => move |scl| {
            win.draw_preview();
        }));
        self.imp().fret_count.connect_value_changed(clone!(@weak self as win => move |scl| {
            win.draw_preview();
        }));
    }

    fn variant(&self) -> Variant {
        match self.imp().variant_box.active() {
            Some(1) => {
                let scale = self.imp().scale_multi.value();
                let hand = Handedness::Right;
                Variant::Multiscale(scale, hand)
            },
            Some(2) => {
                let scale = self.imp().scale_multi.value();
                let hand = Handedness::Left;
                Variant::Multiscale(scale, hand)
            },
            _ => Variant::Monoscale,
        }
    }

    /// Takes the data represented by our Gtk widgets and outputs a Specs struct
    /// which will be used by the backend to render the svg image.
    #[allow(clippy::cast_sign_loss)]
    fn specs(&self) -> Specs {
        Specs::init(
            self.imp().scale.value(),
            self.imp().fret_count.value_as_int() as u32,
            self.variant(),
            self.imp().nut_width.value(),
            match CONFIG.try_lock().unwrap().units {
                Units::Metric => self.imp().bridge_spacing.value() + 6.0,
                Units::Imperial => self.imp().bridge_spacing.value() + (6.0 / 20.4),
            },
            self.imp().perpendicular_fret.value(),
        )
    }

    /// Performs a full render of the svg image without saving to disk, and
    /// refreshes the image preview with the new data.
    fn draw_preview(&self) {
        //let cfg = CONFIG.try_lock().unwrap().clone();
        let cfg = fretboard_layout::Config::default();
        let image = self.specs().create_document(Some(cfg)).to_string();
        let bytes = gtk::glib::Bytes::from_owned(image.into_bytes());
        let stream = MemoryInputStream::from_bytes(&bytes);
        let width = self.imp().image_preview.size(gtk::Orientation::Horizontal);
        let pixbuf =
            Pixbuf::from_stream_at_scale(&stream, width, -1, true, Option::<&Cancellable>::None);
        if let Err(e) = pixbuf {
            eprintln!("{e}");
            return;
        }
        self.imp().image_preview.set_pixbuf(Some(&pixbuf.unwrap()));
        /*if swap {
            if let Ok(mut file) = FILE.try_lock() {
                file.unset_current();
                self.set_window_title(&file);
            }
        }*/
    }

    fn toggle_multi(&self, set: bool) {
        self.imp().scale_multi.set_visible(set);
        self.imp().scale_multi_fine.set_visible(set);
        self.imp().pfret_label.set_visible(set);
        self.imp().perpendicular_fret.set_visible(set);
    }
}
