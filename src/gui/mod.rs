#![warn(clippy::all, clippy::pedantic)]
use clap::crate_version;
use fretboard_layout::Specs;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::{Cancellable, MemoryInputStream};
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, FileChooserAction,
    Inhibit, MessageDialog, ResponseType, Window};
use svg::Document;

use std::path::PathBuf;
use std::rc::Rc;

mod dialogs;
mod file;

use dialogs::Dialogs;
use file::File;

struct Menu {
    app_menu: gtk::Popover,
    template: gtk::Button,
    save: gtk::Button,
    save_as: gtk::Button,
    external: gtk::Button,
    preferences: gtk::Button,
    quit: gtk::Button,
}

struct Gui {
    window: gtk::Window,
    image_preview: gtk::Picture,
    scale: gtk::Scale,
    checkbox_multi: gtk::CheckButton,
    scale_multi_course: gtk::Scale,
    scale_multi_fine: gtk::SpinButton,
    fret_count: gtk::SpinButton,
    pfret_label: gtk::Label,
    perpendicular_fret: gtk::SpinButton,
    nut_width: gtk::SpinButton,
    bridge_spacing: gtk::SpinButton,
    menu: Menu,
    file: File,
    dialogs: Dialogs,
}

impl Menu {
    fn init(builder: &gtk::Builder) -> Menu {
        Menu {
            app_menu: builder.object("app_menu").unwrap(),
            template: builder.object("template").unwrap(),
            save: builder.object("save").unwrap(),
            save_as: builder.object("save_as").unwrap(),
            external: builder.object("external").unwrap(),
            preferences: builder.object("preferences").unwrap(),
            quit: builder.object("quit").unwrap(),
        }
    }
}

impl Gui {
    fn init() -> Gui {
        let builder = gtk::Builder::from_string(include_str!("gui.ui"));
        let window: gtk::Window = builder.object("mainWindow").unwrap();

        Gui {
            window: window.clone(),
            image_preview: builder.object("image_preview").unwrap(),
            scale: builder.object("scale_course").unwrap(),
            checkbox_multi: builder.object("check_box_multi").unwrap(),
            scale_multi_course: builder.object("scale_multi_course").unwrap(),
            scale_multi_fine: builder.object("scale_multi_fine").unwrap(),
            fret_count: builder.object("fret_count").unwrap(),
            perpendicular_fret: builder.object("perpendicular_fret").unwrap(),
            pfret_label: builder.object("pfret_label").unwrap(),
            nut_width: builder.object("nut_width").unwrap(),
            bridge_spacing: builder.object("bridge_spacing").unwrap(),
            menu: Menu::init(&builder),
            file: File::init(),
            dialogs: Dialogs::init(&window, &builder),
        }
    }

    /// Takes the data represented by our Gtk widgets and outputs a Specs struct
    /// which will be used by the backend to render the svg image.
    #[allow(clippy::cast_sign_loss)]
    fn get_specs(&self) -> Specs {
        Specs {
            scale: self.scale.value(),
            count: self.fret_count.value_as_int() as u32,
            multi: self.checkbox_multi.is_active(),
            scale_treble: self.scale_multi_course.value(),
            nut: self.nut_width.value(),
            bridge: self.bridge_spacing.value() + 6.0,
            pfret: self.perpendicular_fret.value(),
        }
    }

    /// Performs a full render of the svg image without saving to disk, and
    /// refreshes the image preview with the new data.
    fn draw_preview(&self, swap: bool) {
        let image = self.get_specs().create_document(None).to_string();
        let bytes = gtk::glib::Bytes::from_owned(image.into_bytes());
        let stream = MemoryInputStream::from_bytes(&bytes);
        let mut width = self.window.size(gtk::Orientation::Horizontal);
        if width == 0 {
            width = 500;
        };
        let pixbuf = Pixbuf::from_stream_at_scale::<MemoryInputStream, Cancellable>(
            &stream, width, 100, true, None,
        );
        self.image_preview.set_pixbuf(Some(&pixbuf.unwrap()));
        if swap {
            self.file.unset_current();
            self.set_window_title();
        }
    }

    fn toggle_multi(&self) {
        let value = self.checkbox_multi.is_active();
        self.scale_multi_course.set_sensitive(value);
        self.scale_multi_fine.set_sensitive(value);
        if value {
            self.perpendicular_fret.show();
            self.pfret_label.show();
        } else {
            self.perpendicular_fret.hide();
            self.pfret_label.hide();
        }
    }

    /// Updates the title of the program window with the name of the output file.
    fn set_window_title(&self) {
        if !self.file.saved() {
            self.window
                .set_title(Some(&format!("Gfret - {} - <unsaved>", crate_version!())));
        } else if self.file.current() {
            if let Some(filename) = self.file.filename() {
                self.window.set_title(Some(&format!(
                    "Gfret - {} - {}",
                    crate_version!(),
                    filename
                )));
            }
        } else {
            if let Some(filename) = self.file.filename() {
                self.window.set_title(Some(&format!(
                    "Gfret - {} - {}*",
                    crate_version!(),
                    filename
                )));
            }
        }
    }
}

pub fn main() {
    let application = gtk::Application::new(Some("org.hitchhiker-linux.gfret"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let gui = Rc::new(Gui::init());
    gui.window
        .set_title(Some(&format!("Gfret - {} - <unsaved>", crate_version!())));

    gui.window.set_application(Some(application));
    gui.toggle_multi();
    gui.draw_preview(false);

    gui.scale
        .connect_value_changed(clone!(@strong gui => move |_| {
            gui.draw_preview(false);
        }));

    gui.scale_multi_course
        .connect_value_changed(clone!(@strong gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.fret_count
        .connect_value_changed(clone!(@strong gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.perpendicular_fret
        .connect_value_changed(clone!(@strong gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.nut_width
        .connect_value_changed(clone!(@strong gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.bridge_spacing
        .connect_value_changed(clone!(@strong gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.checkbox_multi
        .connect_toggled(clone!(@strong gui => move |_| {
            gui.toggle_multi();
            gui.draw_preview(true);
        }));

    gui.menu.template.connect_clicked(clone!(@strong gui => move |_| {
        gui.menu.app_menu.popdown();
        gui.dialogs.open_template.show();
    }));

    gui.menu.save.connect_clicked(clone!(@strong gui => move |_| {
        gui.menu.app_menu.popdown();
        if gui.file.saved() {
            if let Some(filename) = gui.file.filename() {
                let document = gui.get_specs().create_document(None);
                gui.file.do_save(filename, &document);
                gui.set_window_title();
            }
        } else {
            gui.dialogs.save_as.show();
        }
    }));

    gui.menu.save_as.connect_clicked(clone!(@strong gui => move |_| {
        gui.menu.app_menu.popdown();
        gui.dialogs.save_as.show();
    }));

    gui.dialogs.save_as.connect_response(clone!(@strong gui => move |dlg,res| {
        if res == ResponseType::Accept {
            if let Some(file) = dlg.file() {
                if let Some(mut path) = file.path() {
                    path.set_extension("svg");
                    if let Some(filename) = path.to_str() {
                        let document = gui.get_specs().create_document(None);
                        gui.file.do_save(filename.to_string(), &document);
                        gui.set_window_title();
                    }
                }
            }
        }
        dlg.hide();
    }));

    gui.dialogs.open_template.connect_response(clone!(@strong gui => move |dlg,res| {
        dlg.hide();
    }));

    gui.menu.external.connect_clicked(clone!(@strong gui => move |_| {
        gui.menu.app_menu.popdown();
    }));

    gui.menu.preferences.connect_clicked(clone!(@strong gui => move |_| {
        gui.menu.app_menu.popdown();
        gui.dialogs.preferences.show();
    }));

    gui.dialogs.preferences.window().connect_response(clone!(@strong gui => move |dlg,res| {
        if res == ResponseType::Accept {
            gui.dialogs.preferences.save_prefs();
        }
        dlg.hide();
    }));

    gui.menu
        .quit
        .connect_clicked(clone!(@strong gui => move |_| {
            gui.menu.app_menu.popdown();
            //gui.cleanup();
            gui.window.close();
        }));

    gui.window.show();
}
