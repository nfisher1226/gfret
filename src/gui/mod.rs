#![warn(clippy::all, clippy::pedantic)]
use clap::crate_version;
use fretboard_layout::Specs;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::{Cancellable, MemoryInputStream};
use gtk::glib::clone;
use gtk::glib::char::Char;
use gtk::glib::{OptionArg, OptionFlags};
use gtk::prelude::*;
use gtk::{Application, ResponseType};

use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

mod dialogs;
mod file;

use crate::config::GfretConfig;
use crate::template::Template;
use crate::CONFIGDIR;
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
        let cfg = GfretConfig::from_file()
            .unwrap_or(GfretConfig::default())
            .to_config();
        let image = self.get_specs().create_document(Some(cfg)).to_string();
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

    /// Sets widget state to match temmplate
    pub fn load_template(&self, template: &Template) {
        self.scale.set_value(template.scale);
        self.fret_count.set_value(template.count.into());
        if let Some(scale_treble) = template.scale_treble {
            self.scale_multi_course.set_value(scale_treble);
            self.checkbox_multi.set_active(true);
        } else {
            self.checkbox_multi.set_active(false);
        }
        self.toggle_multi();
        self.nut_width.set_value(template.nut);
        self.bridge_spacing.set_value(template.bridge);
        if let Some(pfret) = template.pfret {
            self.perpendicular_fret.set_value(pfret);
        }
    }

    /// Populates an instance of Template from the gui
    #[allow(clippy::cast_sign_loss)]
    fn template_from_gui(&self) -> Template {
        Template {
            scale: self.scale.value(),
            count: self.fret_count.value_as_int() as u32,
            scale_treble: {
                if self.checkbox_multi.is_active() {
                    Some(self.scale_multi_course.value())
                } else {
                    None
                }
            },
            nut: self.nut_width.value(),
            bridge: self.bridge_spacing.value(),
            pfret: Some(self.perpendicular_fret.value()),
        }
    }

    fn save(&self) {
        if self.file.saved() {
            if let Some(filename) = self.file.filename() {
                let cfg = GfretConfig::from_file()
                    .unwrap_or(GfretConfig::default());
                let document = self.get_specs().create_document(Some(cfg.to_config()));
                self.save_template(&filename);
                self.file.do_save(&filename, &document);
                self.set_window_title();
            }
        } else {
            self.dialogs.save_as.show();
        }
    }

    fn save_as(&self, res: ResponseType) {
        if res == ResponseType::Accept {
            if let Some(filename) = self.dialogs.get_save_path() {
                let cfg = GfretConfig::from_file()
                    .unwrap_or(GfretConfig::default())
                    .to_config();
                let document = self.get_specs().create_document(Some(cfg));
                self.save_template(&filename);
                self.file.do_save(&filename, &document);
                self.set_window_title();
            }
        }
    }

    /// Saves a template (toml format) to the specified location
    fn save_template(&self, file: &str) {
        let data: Template = self.template_from_gui();
        data.save_to_file(&PathBuf::from(file));
    }

    fn open_external(&self) {
        if let Some(filename) = self.file.filename() {
            let cfg = GfretConfig::from_file()
                .unwrap_or(GfretConfig::default());
            if let Some(cmd) = cfg.external_program {
                match Command::new(&cmd).args(&[&filename]).spawn() {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }

    /// Saves the program state before exiting
    fn cleanup(&self) {
        let data = self.template_from_gui();
        data.save_statefile();
    }
}

pub fn main() {
    let application = gtk::Application::new(Some("org.hitchhiker-linux.gfret"), Default::default());
    application.add_main_option("template", Char::from(b't'), OptionFlags::NONE, OptionArg::String, "", None);
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let gui = Rc::new(Gui::init());
    let mut statefile = CONFIGDIR.clone();
    statefile.push("state.toml");
    if statefile.exists() {
        if let Some(template) = Template::load_from_file(statefile) {
            gui.load_template(&template);
        }
    }

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
        gui.save();
    }));

    gui.menu.save_as.connect_clicked(clone!(@strong gui => move |_| {
        gui.menu.app_menu.popdown();
        gui.dialogs.save_as.show();
    }));

    gui.dialogs.save_as.connect_response(clone!(@strong gui => move |dlg,res| {
        gui.save_as(res);
        dlg.hide();
    }));

    gui.dialogs.open_template.connect_response(clone!(@strong gui => move |dlg,res| {
        if res == ResponseType::Accept {
            if let Some(path) = gui.dialogs.get_template_path() {
                if let Some(template) = Template::load_from_file(path) {
                    gui.load_template(&template);
                }
            }
        }
        dlg.hide();
    }));

    gui.menu.external.connect_clicked(clone!(@strong gui => move |_| {
        gui.menu.app_menu.popdown();
        if !gui.file.saved() {
            gui.dialogs.save_as.show();
        }
        gui.open_external();
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
        gui.draw_preview(true);
    }));

    gui.menu.quit.connect_clicked(clone!(@strong gui => move |_| {
        gui.menu.app_menu.popdown();
        gui.cleanup();
        gui.window.close();
    }));

    gui.window.show();
}
