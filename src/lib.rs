#![warn(clippy::all, clippy::pedantic)]
// switch to this when it stabilizes
//#![feature(mutex_unlock)]
#![doc = include_str!("../README.md")]

use {
    adw::prelude::*,
    fretboard_layout::{Config, Units},
    gtk::{gdk, gio, prelude::*, subclass::prelude::*},
    //gui::file::File,
    lazy_static::lazy_static,
    std::sync::Mutex,
    window::GfretWindow,
};
/// The cli
pub mod cli;
/// Handles getting the configuration data to and from disk
mod config;
/// Crate specific errors
pub(crate) mod error;
/// The Gtk user interface to gfret.
//pub mod gui;
/// Persistent templates
mod template;
pub(crate) mod theme_switcher;
mod window;

//pub use gui::{dialogs::PrefWidgets, Gui};

lazy_static! {
    static ref CONFIG: Mutex<Config> =
        Mutex::new(config::Config::from_file().unwrap_or_default().truncate());
}

pub fn run_gui() {
    let app = adw::Application::builder()
        .application_id("org.hitchhiker_linux.gfret")
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .register_session(true)
        .build();
    app.connect_startup(|_| {
        adw::init();
        adw::StyleManager::default().set_color_scheme(adw::ColorScheme::Default);
    });
    app.connect_activate(move |app| {
        let window = window::GfretWindow::new(app);
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_str!("style.css").as_bytes());
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
    });
    app.run();
}

/// Switches between imperial and metric units
pub(crate) trait ConvertUnits {
    /// Changes to imperial units
    fn to_imperial(&self);
    /// Changes to metric units
    fn to_metric(&self);
}

impl ConvertUnits for gtk::Adjustment {
    fn to_metric(&self) {
        self.set_lower(self.lower() * 20.4);
        self.set_upper(self.upper() * 20.4);
        self.set_value(self.value() * 20.4);
        self.set_step_increment(1.0);
        self.set_page_increment(5.0);
    }

    fn to_imperial(&self) {
        self.set_lower(self.lower() / 20.4);
        self.set_upper(self.upper() / 20.4);
        self.set_value(self.value() / 20.4);
        self.set_step_increment(0.125);
        self.set_page_increment(0.5);
    }
}

impl ConvertUnits for gtk::SpinButton {
    fn to_metric(&self) {
        self.adjustment().to_metric();
        self.set_digits(2);
    }

    fn to_imperial(&self) {
        self.adjustment().to_imperial();
        self.set_digits(3);
    }
}

/*
impl Convert for PrefWidgets {
    fn to_metric(&self) {
        let mut val = self.border.value();
        let mut adjustment = self.border.adjustment();
        adjustment.set_upper(40.0);
        adjustment.set_step_increment(0.10);
        adjustment.set_page_increment(5.0);
        self.border.set_value(val * 20.4);
        self.border.set_digits(2);

        val = self.line_weight.value();
        adjustment = self.line_weight.adjustment();
        adjustment.set_upper(2.0);
        adjustment.set_step_increment(0.10);
        adjustment.set_page_increment(0.50);
        self.line_weight.set_value(val * 20.4);
        self.line_weight.set_digits(2);
    }

    fn to_imperial(&self) {
        let mut val = self.border.value();
        let mut adjustment = self.border.adjustment();
        adjustment.set_upper(40.0 / 20.4);
        adjustment.set_step_increment(0.01);
        adjustment.set_page_increment(0.10);
        self.border.set_value(val / 20.4);
        self.border.set_digits(3);

        val = self.line_weight.value();
        adjustment = self.line_weight.adjustment();
        adjustment.set_upper(0.098);
        adjustment.set_step_increment(0.01);
        adjustment.set_page_increment(0.05);
        self.line_weight.set_value(val / 20.4);
        self.line_weight.set_digits(3);
    }
}
*/
