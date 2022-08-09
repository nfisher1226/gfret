use {
    crate::{Actions, Window, ConvertUnits, CONFIG},
    adw::subclass::prelude::*,
    fretboard_layout::Units,
    gtk::{gdk, glib, prelude::*, subclass::prelude::*},
};

#[derive(Default)]
pub struct Application {}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = adw::Application;
}

impl ObjectImpl for Application {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl ApplicationImpl for Application {
    fn activate(&self, app: &Self::Type) {
        let window = Window::new(app);
        Actions::default().add(&window, app);
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_str!("../style.css").as_bytes());
        gtk::StyleContext::add_provider_for_display(
            &gdk::Display::default().expect("Cannot get display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        {
            let cfg = CONFIG.try_lock().unwrap().clone();
            if cfg.units == Units::Imperial {
                window.to_imperial();
            }
        }
        window.draw_preview();
        window.present();
    }
}

impl AdwApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
