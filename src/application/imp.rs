use {
    crate::{Actions, Window},
    adw::{
        gio::{PropertyAction, Settings, SettingsBindFlags},
        gtk::{self, gdk, glib, prelude::*, subclass::prelude::*},
        subclass::prelude::*,
        traits::AdwApplicationExt,
    },
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
    fn constructed(&self) {
        self.parent_constructed();
        let instance = self.instance();
        let set_property_action =
            PropertyAction::new("set-theme", &instance.style_manager(), "color-scheme");
        instance.add_action(&set_property_action);
        self.settings
            .bind("theme", &instance.style_manager(), "color-scheme")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
    }
}

impl ApplicationImpl for Application {
    fn activate(&self) {
        let instance = self.instance();
        if instance.windows().is_empty() {
            let window = Window::new(&instance);
            Actions::default().add(&window, &instance);
            let provider = gtk::CssProvider::new();
            provider.load_from_data(include_str!("../style.css").as_bytes());
            gtk::StyleContext::add_provider_for_display(
                &gdk::Display::default().expect("Cannot get display"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
            window.draw_preview();
            window.present();
        }
    }

    fn open(&self, files: &[adw::gio::File], _hint: &str) {
        for file in files {
            let file = file.clone();
            if let Some(path) = file.path() {
                match fretboard_layout::open::open(path) {
                    Ok(specs) => {
                        let win = Window::new(&self.instance());
                        Actions::default().add(&win, &self.instance());
                        let provider = gtk::CssProvider::new();
                        provider.load_from_data(include_str!("../style.css").as_bytes());
                        win.present();
                        gtk::StyleContext::add_provider_for_display(
                            &gdk::Display::default().expect("Cannot get display"),
                            &provider,
                            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                        );
                        win.load_specs(&specs);
                        let base = file.basename().unwrap();
                        win.set_toast(&format!("{} opened", base.display()));
                        *win.imp().file.borrow_mut() = Some(file);
                        win.set_changed(false);
                        win.update_title();
                    }
                    Err(e) => {
                        eprintln!("Error opening file: {e}");
                    }
                }
            }
        }
    }
}

impl AdwApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
