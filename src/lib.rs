#![warn(clippy::all, clippy::pedantic)]
// switch to this when it stabilizes
//#![feature(mutex_unlock)]
#![doc = include_str!("../README.md")]

use {
    adw::prelude::*,
    fretboard_layout::Config,
    gtk::{gio, prelude::*},
    //gui::file::File,
    lazy_static::lazy_static,
    std::sync::Mutex
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
    app.connect_activate(move |app| {
        let window = window::GfretWindow::new(app);
        window.draw_preview();
        window.present();
    });
    app.run();
}

/// Switches between imperial and metric units
pub(crate) trait Convert {
    /// Changes to imperial units
    fn to_imperial(&self);
    /// Changes to metric units
    fn to_metric(&self);
}

/*
impl Convert for Gui {
    fn to_metric(&self) {
        self.adjustments.to_metric();
        self.bridge_spacing
            .set_value(self.bridge_spacing.value() * 20.4);
        self.nut_width.set_value(self.nut_width.value() * 20.4);
        self.scale.set_value(self.scale.value() * 20.4);
        self.scale_multi_fine
            .set_value(self.scale_multi_fine.value() * 20.4);
        self.bridge_spacing.set_digits(2);
        self.nut_width.set_digits(2);
        self.scale_fine.set_digits(2);
        self.scale_multi_fine.set_digits(2);
    }

    fn to_imperial(&self) {
        self.adjustments.to_imperial();
        self.bridge_spacing
            .set_value(self.bridge_spacing.value() / 20.4);
        self.nut_width.set_value(self.nut_width.value() / 20.4);
        self.scale.set_value(self.scale.value() / 20.4);
        self.scale_multi_fine
            .set_value(self.scale_multi_fine.value() / 20.4);
        self.bridge_spacing.set_digits(3);
        self.nut_width.set_digits(3);
        self.scale_fine.set_digits(3);
        self.scale_multi_fine.set_digits(3);
    }
}

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
