mod keys;
use {
    super::Gui,
    crate::FILE,
    gtk::{
        gio::SimpleAction,
        glib::{self, clone},
        prelude::*,
    },
    keys::Keys,
    std::rc::Rc,
};

pub(in crate::gui) struct Actions<'a> {
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
    pub(in crate::gui) fn add(&self, gui: &Rc<Gui>, app: &gtk::Application) {
        let keys = Keys::from_file().unwrap_or_default();
        for name in &self.names {
            let action = SimpleAction::new(name, None);
            app.set_accels_for_action(&format!("win.{name}"), &[keys.get(name)]);
            gui.window.add_action(&action);
            match *name {
                "open_template" => {
                    action.connect_activate(clone!(@weak gui => move |_, _| {
                        gui.dialogs.open_template.show();
                    }));
                }
                "save" => {
                    action.connect_activate(clone!(@weak gui => move |_, _| {
                        gui.save();
                    }));
                }
                "save_as" => {
                    action.connect_activate(clone!(@weak gui => move |_, _| {
                        gui.dialogs.save_as.show();
                    }));
                }
                "open_external" => {
                    action.connect_activate(clone!(@weak gui => move |_, _| {
                        if let Ok(file) = FILE.try_lock() {
                            if !file.saved() {
                                gui.dialogs.save_as.show();
                            }
                            Gui::open_external(&file);
                        };
                    }));
                }
                "preferences" => {
                    action.connect_activate(clone!(@weak gui => move |_, _| {
                        gui.dialogs.preferences.show();
                    }));
                }
                "about" => {
                    action.connect_activate(clone!(@strong gui => move |_, _| {
                        gui.dialogs.about.show();
                    }));
                }
                "quit" => {
                    action.connect_activate(clone!(@weak gui => move |_, _| {
                        gui.cleanup();
                        gui.window.close();
                    }));
                }
                _ => unreachable!(),
            }
        }
    }
}
