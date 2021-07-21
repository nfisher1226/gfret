#![warn(clippy::all, clippy::pedantic)]
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, FileChooserAction,
    Inhibit, MessageDialog, ResponseType, Window};

pub struct Dialogs {
    pub save_as: gtk::FileChooserDialog,
    pub open_template: gtk::FileChooserDialog,
}

impl Dialogs {
    pub fn init(window: &gtk::Window) -> Dialogs {
        Dialogs {
            save_as: Dialogs::init_save_as(window),
            open_template: Dialogs::init_open_template(window),
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
}
