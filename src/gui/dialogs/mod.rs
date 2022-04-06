#![warn(clippy::all, clippy::pedantic)]
use {
    crate::config::GfretConfig,
    fretboard_layout::{Color::Reduced, ConvertColor, Font, FontWeight, ReducedRGBA, Units},
    gtk::{pango::FontDescription, prelude::*, DialogFlags, ResponseType},
    std::{env, path::PathBuf, str::FromStr},
};

/// Handles on the widgets in the preferences dialog window for which we need to
/// save data
#[derive(Clone)]
pub struct PrefWidgets {
    window: gtk::Dialog,
    units: gtk::ComboBoxText,
    external_entry: gtk::Entry,
    external_button: gtk::Button,
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

#[derive(Clone)]
pub struct Dialogs {
    pub about: gtk::AboutDialog,
    pub save_as: gtk::FileChooserDialog,
    pub open_template: gtk::FileChooserDialog,
    pub app_chooser: gtk::AppChooserDialog,
    pub preferences: PrefWidgets,
}

impl Dialogs {
    pub fn init(window: &gtk::ApplicationWindow, builder: &gtk::Builder) -> Self {
        let prefs = Self::init_preferences(window, builder);

        let dialogs = Self {
            about: Self::init_about(window),
            save_as: Self::init_save_as(window),
            open_template: Self::init_open_template(window),
            app_chooser: Self::init_app_chooser(&prefs.window),
            preferences: prefs,
        };

        let chooser = dialogs.app_chooser.clone();
        dialogs
            .preferences
            .external_button
            .connect_clicked(move |_| {
                chooser.show();
            });

        let preferences = dialogs.preferences.clone();
        dialogs.app_chooser.connect_response(move |dlg, res| {
            if res == ResponseType::Ok {
                if let Some(app_info) = dlg.app_info() {
                    if let Some(txt) = app_info.commandline().map(|cmd| {
                        String::from(cmd.to_str().unwrap())
                            .split_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_string()
                    }) {
                        preferences.set_external(&txt);
                    }
                };
            };
            dlg.hide();
        });

        dialogs
    }

    fn init_about(window: &gtk::ApplicationWindow) -> gtk::AboutDialog {
        gtk::AboutDialog::builder()
            .program_name("Gfret")
            .authors(vec!["Nathan Fisher".to_string()])
            .version(env!("CARGO_PKG_VERSION"))
            .license(include_str!(r"../../../LICENSE"))
            .wrap_license(true)
            .comments("A tool for lutherie\nBuilt using Rust and Gtk+")
            .logo_icon_name("gfret")
            .copyright("©2020 by Nathan Fisher (the JeanG3nie)")
            .website("https://codeberg.org/jeang3nie/gfret")
            .transient_for(window)
            .build()
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

    fn init_app_chooser(window: &gtk::Dialog) -> gtk::AppChooserDialog {
        gtk::AppChooserDialog::for_content_type(Some(window), DialogFlags::all(), "image/svg+xml")
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
    fn init(builder: &gtk::Builder) -> Self {
        let ui_src = include_str!("prefs.ui");
        builder.add_from_string(ui_src).unwrap();
        PrefWidgets {
            window: builder
                .object("prefs_window")
                .expect("Error getting 'prefs_window'"),
            units: builder
                .object("combo_box_units")
                .expect("Error getting 'units'"),
            external_entry: builder
                .object("external_entry")
                .expect("Error getting 'external_entry'"),
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

    fn set_external(&self, txt: &str) {
        self.external_entry.buffer().set_text(txt);
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

    fn external(&self) -> String {
        self.external_entry.buffer().text()
    }

    /// Converts the value stored in a `gtk::ColorButton` from a `gtk::gdk::RGBA`
    /// struct into a struct which can be serialized using serde
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn color(button: &gtk::ColorButton) -> ReducedRGBA {
        let color = button.rgba();
        ReducedRGBA {
            red: (color.red() * 255.0) as u8,
            green: (color.green() * 255.0) as u8,
            blue: (color.blue() * 255.0) as u8,
            alpha: (color.alpha() * 255.0) as u8,
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
    pub fn config_from_widgets(&self) -> GfretConfig {
        GfretConfig {
            units: self.units(),
            external_program: Some(self.external()),
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
