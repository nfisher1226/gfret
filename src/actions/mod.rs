mod keys;
use {
    crate::{Application, Window},
    adw::gtk::{
        gio::SimpleAction,
        glib::{self, clone},
        prelude::*,
    },
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
                        win.save();
                    }));
                }
                "save_as" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        win.save_as();
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
                        win.run_preferences();
                    }));
                }
                "about" => {
                    action.connect_activate(clone!(@strong win, @weak app => move |_, _| {
                        let win = adw::AboutWindow::builder()
                            .application_icon("gfret")
                            .application_name(&env!("CARGO_PKG_NAME").to_uppercase())
                            .comments("A tool for lutherie\nBuilt using Rust and Gtk+")
                            .copyright("©2020 by Nathan Fisher (the JeanG3nie)")
                            .developer_name("Nathan Fisher")
                            .issue_url("https://codeberg.org/jeang3nie/gfret/issues")
                            .license_type(adw::gtk::License::Bsd)
                            .release_notes("<p>Unreleased</p>\
                                <ul>\
                                <li>Move some common code into lib.rs</li>\
                                <li>Create trait `ConvertUnits` to swap imperial and metric values</li>\
                                <li>Move action handling into module</li>\
                                <li>Move keybindings into module</li>\
                                <li>Make keybindings configurable</li>\
                                <li>Depend on libadwaita</li>\
                                <li>Subclass Application from AdwApplication</li>\
                                <li>Subclass Window from AdwWindow</li>\
                                <li>Use adwaita AboutWindow</li>\
                                </ul>"
                            )
                            .version(env!("CARGO_PKG_VERSION"))
                            .website("https://jeang3nie.codeberg.page/gfret/")
                            .application(&app)
                            .transient_for(&win)
                            .build();
                        win.show();
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
