#![warn(clippy::all, clippy::pedantic)]
use fretboard_layout::config::{Font, FontWeight, Units};
use gtk::pango::FontDescription;
use gtk::prelude::*;
use rgba_simple::{Color::Reduced, Convert, ReducedRGBA};

use crate::config::GfretConfig;

use std::env;
use std::path::PathBuf;
use std::str::FromStr;

/// Handles on the widgets in the preferences dialog window for which we need to
/// save data
#[derive(Clone)]
pub struct PrefWidgets {
    window: gtk::Dialog,
    units: gtk::ComboBoxText,
    external_button: gtk::AppChooserButton,
    border: gtk::SpinButton,
    line_weight: gtk::SpinButton,
    fretline_color: gtk::ColorButton,
    fretboard_color: gtk::ColorButton,
    draw_centerline: gtk::Switch,
    centerline_box: gtk::Box,
    centerline_color: gtk::ColorButton,
    print_specs: gtk::Switch,
    font_box: gtk::Box,
    font_chooser: gtk::FontButton,
}

pub struct Dialogs {
    pub about: gtk::AboutDialog,
    pub save_as: gtk::FileChooserDialog,
    pub open_template: gtk::FileChooserDialog,
    pub preferences: PrefWidgets,
}

impl Dialogs {
    pub fn init(window: &gtk::ApplicationWindow, builder: &gtk::Builder) -> Dialogs {
        Dialogs {
            about: Dialogs::init_about(window),
            save_as: Dialogs::init_save_as(window),
            open_template: Dialogs::init_open_template(window),
            preferences: Dialogs::init_preferences(window, builder),
        }
    }

    fn init_about(window: &gtk::ApplicationWindow) -> gtk::AboutDialog {
        let dlg = gtk::AboutDialog::builder()
            .program_name("Gfret")
            .authors(vec!("Nathan Fisher".to_string()))
            .version(env!("CARGO_PKG_VERSION"))
            .license(include_str!(r"../../../LICENSE"))
            .wrap_license(true)
            .comments("A tool for lutherie\nBuilt using Rust and Gtk+")
            .logo_icon_name("gfret")
            .copyright("©2020 by Nathan Fisher (the JeanG3nie)")
            .website("https://codeberg.org/jeang3nie/gfret")
            .transient_for(window)
            .build();
        dlg
    }

    fn init_save_as(window: &gtk::ApplicationWindow) -> gtk::FileChooserDialog {
        let dlg = gtk::FileChooserDialog::builder()
            .action(gtk::FileChooserAction::Save)
            .name("Gfret - Save As")
            .use_header_bar(1)
            .create_folders(true)
            .select_multiple(false)
            .modal(true)
            .destroy_with_parent(true)
            .transient_for(window)
            .build();
        let accept = gtk::Button::with_label("Accept");
        let cancel = gtk::Button::with_label("Cancel");
        dlg.add_action_widget(&cancel, gtk::ResponseType::Cancel);
        dlg.add_action_widget(&accept, gtk::ResponseType::Accept);
        dlg
    }

    fn init_open_template(window: &gtk::ApplicationWindow) -> gtk::FileChooserDialog {
        let dlg = gtk::FileChooserDialog::builder()
            .action(gtk::FileChooserAction::Open)
            .name("Gfret - Open Template")
            .use_header_bar(1)
            .create_folders(true)
            .select_multiple(false)
            .modal(true)
            .destroy_with_parent(true)
            .transient_for(window)
            .build();
        let accept = gtk::Button::with_label("Accept");
        let cancel = gtk::Button::with_label("Cancel");
        dlg.add_action_widget(&cancel, gtk::ResponseType::Cancel);
        let filter = gtk::FileFilter::new();
        filter.add_pattern("*.toml");
        filter.set_name(Some("toml files"));
        dlg.add_filter(&filter);
        dlg.add_action_widget(&accept, gtk::ResponseType::Accept);
        dlg
    }

    fn init_preferences(window: &gtk::ApplicationWindow, builder: &gtk::Builder) -> PrefWidgets {
        let dlg = PrefWidgets::init(builder);
        dlg.load_config();
        let accept = gtk::Button::with_label("Accept");
        let cancel = gtk::Button::with_label("Cancel");
        dlg.window
            .add_action_widget(&cancel, gtk::ResponseType::Cancel);
        dlg.window
            .add_action_widget(&accept, gtk::ResponseType::Accept);
        dlg.window.set_transient_for(Some(window));
        let dlg_clone = dlg.clone();
        dlg.units.connect_changed(move |_| {
            match dlg_clone.units() {
                Units::Metric => dlg_clone.to_metric(),
                Units::Imperial => dlg_clone.to_imperial(),
            };
        });
        let dlg_clone = dlg.clone();
        dlg.draw_centerline.connect_state_set(move |_, _| {
            dlg_clone.toggle_centerline_color();
            gtk::Inhibit(false)
        });
        let dlg_clone = dlg.clone();
        dlg.print_specs.connect_state_set(move |_, _| {
            dlg_clone.toggle_font_chooser();
            gtk::Inhibit(false)
        });
        dlg
    }

    pub fn get_template_path(&self) -> Option<PathBuf> {
        if let Some(file) = self.open_template.file() {
            if let Some(path) = file.path() {
                return Some(path);
            }
        }
        None
    }

    pub fn get_save_path(&self) -> Option<String> {
        if let Some(file) = self.save_as.file() {
            if let Some(mut path) = file.path() {
                path.set_extension("svg");
                if let Some(filename) = path.to_str() {
                    return Some(String::from(filename));
                }
            }
        }
        None
    }
}

impl PrefWidgets {
    /// Returns a struct of pointers to the widgets that contain state
    fn init(builder: &gtk::Builder) -> PrefWidgets {
        let ui_src = include_str!("prefs.ui");
        builder.add_from_string(ui_src).unwrap();
        PrefWidgets {
            window: builder
                .object("prefs_window")
                .expect("Error getting 'prefs_window'"),
            units: builder
                .object("combo_box_units")
                .expect("Error getting 'units'"),
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
            centerline_box: builder
                .object("centerline_box")
                .expect("Error getting 'centerline_box'"),
            centerline_color: builder
                .object("centerline_color")
                .expect("Error getting 'centerline_color'"),
            print_specs: builder
                .object("print_specs")
                .expect("Error getting 'print_specs'"),
            font_box: builder
                .object("font_box")
                .expect("Error getting 'font_box'"),
            font_chooser: builder
                .object("font_chooser")
                .expect("Error getting 'font_chooser'"),
        }
    }

    pub fn show(&self) {
        self.window.show();
    }

    pub fn window(&self) -> gtk::Dialog {
        self.window.clone()
    }

    fn units(&self) -> Units {
        match self.units.active() {
            Some(1) => Units::Imperial,
            Some(_) | None => Units::Metric,
        }
    }

    fn to_metric(&self) {
        let mut val = self.border.value();
        let mut adjustment = self.border.adjustment();
        adjustment.set_upper(40.0);
        adjustment.set_step_increment(0.10);
        adjustment.set_page_increment(5.0);
        self.border.set_value(val * 20.4);

        val = self.line_weight.value();
        adjustment = self.line_weight.adjustment();
        adjustment.set_upper(2.0);
        adjustment.set_step_increment(0.10);
        adjustment.set_page_increment(0.50);
        self.line_weight.set_value(val * 20.4);
    }

    fn to_imperial(&self) {
        let mut val = self.border.value();
        let mut adjustment = self.border.adjustment();
        adjustment.set_upper(40.0 / 20.4);
        adjustment.set_step_increment(0.01);
        adjustment.set_page_increment(0.10);
        self.border.set_value(val / 20.4);

        val = self.line_weight.value();
        adjustment = self.line_weight.adjustment();
        adjustment.set_upper(0.098);
        adjustment.set_step_increment(0.01);
        adjustment.set_page_increment(0.05);
        self.line_weight.set_value(val / 20.4);
    }

    /// Retreives the commandline for the external editor
    fn external(&self) -> Option<String> {
        if let Some(app_info) = self.external_button.app_info() {
            app_info.commandline().map(|cmd| {
                String::from(cmd.to_str().unwrap())
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string()
            })
        } else {
            None
        }
    }

    /// Converts the value stored in a `gtk::ColorButton` from a `gtk::gdk::RGBA`
    /// struct into a struct which can be serialized using serde
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn color(button: &gtk::ColorButton) -> ReducedRGBA {
        let color = button.rgba();
        ReducedRGBA {
            red: (color.red * 255.0) as u8,
            green: (color.green * 255.0) as u8,
            blue: (color.blue * 255.0) as u8,
            alpha: (color.alpha * 255.0) as u8,
        }
    }

    fn font(&self) -> Option<Font> {
        match self.font_chooser.font() {
            Some(f) => {
                let font = FontDescription::from_string(&f);
                let font_family = match font.family() {
                    Some(fam) => fam.to_string(),
                    None => String::from("Sans"),
                };
                let font_weight = font.style().to_string();
                Some(Font {
                    family: font_family,
                    weight: {
                        match FontWeight::from_str(&font_weight) {
                            Ok(w) => w,
                            Err(e) => {
                                eprintln!("Error parsing FontWeight: {}", e);
                                eprintln!("FontWeight was: {}", &font_weight);
                                FontWeight::default()
                            }
                        }
                    },
                })
            }
            None => None,
        }
    }

    /// Returns a `GfretConfig` struct from the widget states
    fn config_from_widgets(&self) -> GfretConfig {
        GfretConfig {
            units: self.units(),
            external_program: self.external(),
            border: self.border.value(),
            line_weight: self.line_weight.value(),
            fretline_color: Reduced(PrefWidgets::color(&self.fretline_color)),
            fretboard_color: Reduced(PrefWidgets::color(&self.fretboard_color)),
            centerline_color: Some(Reduced(PrefWidgets::color(&self.centerline_color))),
            font: self.font(),
        }
    }

    /// Sets widget states based on a `GfretConfig` struct which is loaded from file
    fn load_config(&self) {
        if let Some(config) = GfretConfig::from_file() {
            if let Ok(color) = &config.fretline_color.to_gdk() {
                self.fretline_color.set_rgba(color);
            }
            if let Ok(color) = &config.fretboard_color.to_gdk() {
                self.fretboard_color.set_rgba(color);
            }
            if let Some(c) = config.centerline_color {
                self.draw_centerline.set_active(true);
                if let Ok(color) = &c.to_gdk() {
                    self.centerline_color.set_sensitive(true);
                    self.centerline_color.set_rgba(color);
                }
            } else {
                self.draw_centerline.set_active(false);
                self.centerline_color.set_sensitive(false);
            }
            match config.units {
                Units::Metric => self.units.set_active(Some(0)),
                Units::Imperial => self.units.set_active(Some(1)),
            };
            self.border.set_value(config.border);
            self.line_weight.set_value(config.line_weight);
            if let Some(f) = config.font {
                self.print_specs.set_active(true);
                self.font_chooser.set_sensitive(true);
                self.font_chooser
                    .set_font(&format!("{} {}", f.family, f.weight));
            } else {
                self.print_specs.set_active(false);
                self.font_chooser.set_sensitive(false);
            }
        }
    }

    /// Serializes a `GfretConfig` struct as toml and saves to disk
    pub fn save_prefs(&self) {
        let config_file = crate::config::get_config_file();
        let config_data = self.config_from_widgets();
        config_data.save_to_file(&config_file);
    }

    /// Toggles the centerline color chooser button
    fn toggle_centerline_color(&self) {
        if self.draw_centerline.is_active() {
            self.centerline_box.show();
        } else {
            self.centerline_box.hide();
        }
    }

    /// Toggles the font chooser button
    fn toggle_font_chooser(&self) {
        if self.print_specs.is_active() {
            self.font_box.show();
        } else {
            self.font_box.hide();
        }
    }
}
