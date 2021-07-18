#![warn(clippy::all, clippy::pedantic)]
use fretboard_layout::color::ReducedRGBA;
use gtk::glib::clone;
use gtk::prelude::*;

//use crate::Config;

use std::rc::Rc;
use std::str::FromStr;

/// Opens a [gtk::AppChooserDialog]
//mod appchooser;

/// Handles on the widgets in the preferences dialog window for which we need to
/// save data
struct PrefWidgets {
    prefs_window: gtk::Dialog,
    external_program: gtk::Entry,
    external_button: gtk::Button,
    border: gtk::SpinButton,
    line_weight: gtk::SpinButton,
    fretline_color: gtk::ColorButton,
    fretboard_color: gtk::ColorButton,
    draw_centerline: gtk::Switch,
    centerline_color: gtk::ColorButton,
    print_specs: gtk::Switch,
    font_chooser: gtk::FontButton,
    background_color: gtk::ColorButton,
}

impl PrefWidgets {
    /// Returns a struct of pointers to the widgets that contain state
    fn new() -> PrefWidgets {
        let glade_src = include_str!("prefs.ui");
        let builder = gtk::Builder::from_string(glade_src);
        PrefWidgets {
            prefs_window: builder
                .object("prefs_window")
                .expect("Error getting 'prefs_window'"),
            external_program: builder
                .object("external_program")
                .expect("Error getting 'external_program'"),
            external_button: builder
                .object("external_button")
                .expect("Error getting 'external_button'"),
            border: builder.object("border").expect("Error getting 'border'"),
            line_weight: builder
                .object("line_weight")
                .expect("Error getting 'line_weight'"),
            fretline_color: builder
                .object("fretline_color")
                .expect("Error getting 'fretline_color'"),
            fretboard_color: builder
                .object("fretboard_color")
                .expect("Error getting 'fretboard_color'"),
            draw_centerline: builder
                .object("draw_centerline")
                .expect("Error getting 'draw_centerline'"),
            centerline_color: builder
                .object("centerline_color")
                .expect("Error getting 'centerline_color'"),
            print_specs: builder
                .object("print_specs")
                .expect("Error getting 'print_specs'"),
            font_chooser: builder
                .object("font_chooser")
                .expect("Error getting 'font_chooser'"),
            background_color: builder
                .object("background_color")
                .expect("Error getting 'background_color'"),
        }
    }

    /// Converts the value stored in a [gtk::ColorButton] from a [gdk::RGBA]
    /// struct into a String suitable for saving in config.toml
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn get_color(button: &gtk::ColorButton) -> ReducedRGBA {
        let color = button.rgba();
        ReducedRGBA {
            red: (color.red * 255.0) as u8,
            green: (color.green * 255.0) as u8,
            blue: (color.blue * 255.0) as u8,
            alpha: (color.alpha * 255.0) as u8,
        }
    }

    /// Returns a [Config] struct from the widget states
    /*fn config_from_widgets(&self) -> Config {
        Config {
            external_program: String::from(self.external_program.get_text()),
            border: self.border.get_value(),
            line_weight: self.line_weight.get_value(),
            fretline_color: PrefWidgets::get_color_string(&self.fretline_color),
            fretboard_color: PrefWidgets::get_color_string(&self.fretboard_color),
            draw_centerline: self.draw_centerline.get_active(),
            centerline_color: PrefWidgets::get_color_string(&self.centerline_color),
            print_specs: self.print_specs.get_active(),
            font: {
                match self.font_chooser.get_font() {
                    Some(c) => Some(String::from(c)),
                    None => None,
                }
            },
            background_color: PrefWidgets::get_color_string(&self.background_color),
        }
    }*/

    /// Sets widget states based on a [Config] struct which is loaded from file
    /*fn load_config(&self) {
        if let Some(config) = Config::from_file() {
            if let Ok(color) = gdk::RGBA::from_str(&config.fretline_color) {
                self.fretline_color.set_rgba(&color);
            }
            if let Ok(color) = gdk::RGBA::from_str(&config.centerline_color) {
                self.centerline_color.set_rgba(&color);
            }
            if let Ok(color) = gdk::RGBA::from_str(&config.fretboard_color) {
                self.fretboard_color.set_rgba(&color);
            }
            if let Ok(color) = gdk::RGBA::from_str(&config.background_color) {
                self.background_color.set_rgba(&color);
            }
            self.external_program.set_text(&config.external_program);
            self.border.set_value(config.border);
            self.line_weight.set_value(config.line_weight);
            self.draw_centerline.set_active(config.draw_centerline);
            self.centerline_color.set_sensitive(config.draw_centerline);
            self.print_specs.set_active(config.print_specs);
            self.font_chooser.set_sensitive(config.print_specs);
            if let Some(font) = config.font {
                self.font_chooser.set_font(&font);
            }
        }
    }*/

    /// Serializes a [Config] struct as toml and saves to disk
    /*fn save_prefs(&self) {
        let config_file = Config::get_config_file();
        let config_data = self.config_from_widgets();
        config_data.save_to_file(&config_file);
    }*/

    /// Toggles the centerline color chooser button
    fn toggle_centerline_color(&self) {
        let state = self.draw_centerline.is_active();
        self.centerline_color.set_sensitive(state);
    }

    /// Toggles the font chooser button
    fn toggle_font_chooser(&self) {
        let state = self.print_specs.is_active();
        self.font_chooser.set_sensitive(state);
    }
}

/// Runs the preferences dialog
pub fn run() {
    let prefs = Rc::new(PrefWidgets::new());
    /*prefs.load_config();
    prefs
        .external_program
        .connect_changed(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    prefs
        .external_button
        .connect_clicked(clone!(@weak prefs => move |_| {
            let command = appchooser::run();
            if command.is_some() {
                prefs.external_program.set_text(&command.unwrap());
            }
        }));

    prefs
        .border
        .connect_value_changed(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    prefs
        .line_weight
        .connect_value_changed(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    prefs
        .fretline_color
        .connect_color_set(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    prefs
        .fretboard_color
        .connect_color_set(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    let prefs_clone = prefs.clone();
    prefs
        .draw_centerline
        .connect_state_set(move |_, _| {
            prefs_clone.toggle_centerline_color();
            prefs_clone.save_prefs();
            gtk::Inhibit(false)
        });

    prefs
        .centerline_color
        .connect_color_set(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    let prefs_clone = prefs.clone();
    prefs
        .print_specs
        .connect_state_set(move |_, _| {
            prefs_clone.toggle_font_chooser();
            prefs_clone.save_prefs();
            gtk::Inhibit(false)
        });

    prefs
        .font_chooser
        .connect_font_set(clone!(@weak prefs => move |_| {
            if prefs.font_chooser.get_font().is_some() {
                prefs.save_prefs();
            }
        }));

    prefs
        .background_color
        .connect_color_set(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));*/

    prefs
        .prefs_window
        .connect_response(clone!(@strong prefs => move |_,response| {
            if response == gtk::ResponseType::Ok {
                println!("Response accepted");
                prefs.prefs_window.close();
            }
        }));

    prefs.prefs_window.show();
}
