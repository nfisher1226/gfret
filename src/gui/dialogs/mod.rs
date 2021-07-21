#![warn(clippy::all, clippy::pedantic)]
use fretboard_layout::color::ReducedRGBA;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, FileChooserAction,
    Inhibit, MessageDialog, ResponseType, Window};


/// Handles on the widgets in the preferences dialog window for which we need to
/// save data
pub struct PrefWidgets {
    window: gtk::Dialog,
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

pub struct Dialogs {
    pub save_as: gtk::FileChooserDialog,
    pub open_template: gtk::FileChooserDialog,
    pub preferences: PrefWidgets,
}

impl Dialogs {
    pub fn init(window: &gtk::Window, builder: &gtk::Builder) -> Dialogs {
        Dialogs {
            save_as: Dialogs::init_save_as(window),
            open_template: Dialogs::init_open_template(window),
            preferences: Dialogs::init_preferences(window, builder),
        }
    }

    fn init_save_as(window: &gtk::Window) -> gtk::FileChooserDialog {
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

    fn init_open_template(window: &gtk::Window) -> gtk::FileChooserDialog {
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
        dlg.add_action_widget(&accept, gtk::ResponseType::Accept);
        dlg
    }

    fn init_preferences(window: &gtk::Window, builder: &gtk::Builder) -> PrefWidgets {
        let dlg = PrefWidgets::init(builder);
        let accept = gtk::Button::with_label("Accept");
        let cancel = gtk::Button::with_label("Cancel");
        dlg.window.add_action_widget(&cancel, gtk::ResponseType::Cancel);
        dlg.window.add_action_widget(&accept, gtk::ResponseType::Accept);
        dlg.window.set_transient_for(Some(window));
        dlg
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

    pub fn show(&self) {
        self.window.show();
    }

    pub fn hide(&self) {
        self.window.hide();
    }

    pub fn window(&self) -> gtk::Dialog {
        self.window.clone()
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

