mod keys;
use {
    adw::gtk::{
        gio::SimpleAction,
        glib::{self, clone},
        prelude::*,
    },
    crate::{Application, Window},
    keys::Keys,
};

pub struct Actions<'a> {
    names: [&'a str; 7],
}

impl<'a> Default for Actions<'a> {
    fn default() -> Self {
        Self {
            names: [
                "open_template",
                "save",
                "save_as",
                "open_external",
                "preferences",
                "about",
                "quit",
            ],
        }
    }
}

impl<'a> Actions<'a> {
    pub fn add(&self, win: &Window, app: &Application) {
        let keys = Keys::from_file().unwrap_or_default();
        for name in &self.names {
            let action = SimpleAction::new(name, None);
            app.set_accels_for_action(&format!("win.{name}"), &[keys.get(name)]);
            win.add_action(&action);
            match *name {
                "open_template" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        //gui.dialogs.open_template.show();
                    }));
                }
                "save" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        //gui.save();
                    }));
                }
                "save_as" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        //gui.dialogs.save_as.show();
                    }));
                }
                "open_external" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        /*if let Ok(file) = FILE.try_lock() {
                            if !file.saved() {
                                gui.dialogs.save_as.show();
                            }
                            Gui::open_external(&file);
                        };*/
                    }));
                }
                "preferences" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        //gui.dialogs.preferences.show();
                    }));
                }
                "about" => {
                    action.connect_activate(clone!(@strong win => move |_, _| {
                        adw::gtk::show_about_dialog(Some(&win), &[
                            ("program-name", &"Gfret".to_value()),
                            ("authors", &["Nathan Fisher"].to_value()),
                            ("version", &env!("CARGO_PKG_VERSION")),
                            ("license", &include_str!(r"../../LICENSE")),
                            ("wrap-license", &true),
                            ("comments", &"A tool for lutherie\nBuilt using Rust and Gtk+"),
                            ("logo-icon-name", &"gfret"),
                            ("copyright", &"©2020 by Nathan Fisher (the JeanG3nie)"),
                            ("website", &"https://codeberg.org/jeang3nie/gfret"),
                        ]);
                    }));
                }
                "quit" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        //gui.cleanup();
                        win.close();
                    }));
                }
                _ => unreachable!(),
            }
        }
    }
}
