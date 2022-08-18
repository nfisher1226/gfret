use {
    crate::{Actions, ConvertUnits, Window, CONFIG},
    adw::{
        gio::{PropertyAction, Settings, SettingsBindFlags},
        gtk::{self, gdk, glib, prelude::*, subclass::prelude::*},
        subclass::prelude::*,
        traits::AdwApplicationExt,
    },
    fretboard_layout::Units,
};

pub struct Application {
    pub settings: Settings,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            settings: Settings::new("org.hitchhiker_linux.gfret"),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = adw::Application;
}

impl ObjectImpl for Application {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let set_property_action =
            PropertyAction::new("set-theme", &obj.style_manager(), "color-scheme");
        obj.add_action(&set_property_action);
        self.settings
            .bind("theme", &obj.style_manager(), "color-scheme")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
    }
}

impl ApplicationImpl for Application {
    fn activate(&self, app: &Self::Type) {
        if app.windows().is_empty() {
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
}

impl AdwApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
