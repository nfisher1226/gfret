use clap::crate_version;
use fretboard_layout::Specs;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::{Cancellable, MemoryInputStream};
use gtk::prelude::*;
use gtk::glib::clone;
use gtk::{Application, ApplicationWindow, Builder, Button, Inhibit, MessageDialog, ResponseType};

use std::cell::RefCell;
use std::rc::Rc;

struct Menu {
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
}

impl Menu {
    fn init(builder: gtk::Builder) -> Menu {
        Menu {
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

        Gui {
            window: builder.object("mainWindow").unwrap(),
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
            menu: Menu::init(builder),
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
        let mut width = self.image_preview.size(gtk::Orientation::Horizontal);
        if width == 0 { width = 1000 };
        let pixbuf = Pixbuf::from_stream_at_scale::<MemoryInputStream, Cancellable>(
            &stream,
            width,
            -1,
            true,
            None,
        );
        self.image_preview.set_pixbuf(Some(&pixbuf.unwrap()));
        //if swap {
        //    self.saved_current.swap(&RefCell::new(false));
        //    self.set_window_title();
        //}
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
}


pub fn main() {
    let application = gtk::Application::new(
        Some("org.hitchhiker-linux.gfret"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let gui = Rc::new(Gui::init());
    gui.window
        .set_title(Some(&format!("Gfret - {} - <unsaved>", crate_version!())));

    gui.window.set_application(Some(application));
    //gui.setup_menu();
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
            gui.draw_preview(false);
        }));

    gui.menu.quit
        .connect_clicked(clone!(@strong gui => move |_| {
        //gui.cleanup();
            gui.window.close();
        }));

    gui.window.show();
}
