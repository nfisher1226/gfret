#![warn(clippy::all, clippy::pedantic)]
mod adjustments;
mod dialogs;
pub mod file;

use {
    crate::{
        config::{self, Config},
        template::Template,
        Convert, CONFIG, FILE,
    },
    adjustments::Adjustments,
    dialogs::Dialogs,
    file::File,
    fretboard_layout::{Handedness, Specs, Units, Variant},
    gtk::{
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream, SimpleAction},
        glib,
        glib::{char::Char, clone, MainContext, OptionArg, OptionFlags, PRIORITY_DEFAULT},
        prelude::*,
        Application, ResponseType,
    },
    std::{mem, path::PathBuf, process::Command, rc::Rc, string::ToString, thread},
};

struct Gui {
    window: gtk::ApplicationWindow,
    image_preview: gtk::Picture,
    scale: gtk::Scale,
    scale_fine: gtk::SpinButton,
    variant: gtk::ComboBox,
    handedness: gtk::ComboBox,
    scale_multi_course: gtk::Scale,
    scale_multi_fine: gtk::SpinButton,
    fret_count: gtk::SpinButton,
    pfret_label: gtk::Label,
    perpendicular_fret: gtk::SpinButton,
    nut_width: gtk::SpinButton,
    bridge_spacing: gtk::SpinButton,
    dialogs: Dialogs,
    adjustments: Adjustments,
}

struct Actions {
    open_template: SimpleAction,
    save: SimpleAction,
    save_as: SimpleAction,
    open_external: SimpleAction,
    preferences: SimpleAction,
    about: SimpleAction,
    quit: SimpleAction,
}

impl Actions {
    fn init() -> Actions {
        Actions {
            open_template: SimpleAction::new("open_template", None),
            save: SimpleAction::new("save", None),
            save_as: SimpleAction::new("save_as", None),
            open_external: SimpleAction::new("open_external", None),
            preferences: SimpleAction::new("preferences", None),
            about: SimpleAction::new("about", None),
            quit: SimpleAction::new("quit", None),
        }
    }

    fn connect(&self, gui: &Rc<Gui>) {
        self.open_template
            .connect_activate(clone!(@weak gui => move |_, _| {
                gui.dialogs.open_template.show();
            }));

        self.save.connect_activate(clone!(@weak gui => move |_, _| {
            gui.save();
        }));

        self.save_as
            .connect_activate(clone!(@weak gui => move |_, _| {
                gui.dialogs.save_as.show();
            }));

        self.open_external
            .connect_activate(clone!(@weak gui => move |_, _| {
                if let Ok(file) = FILE.try_lock() {
                    if !file.saved() {
                        gui.dialogs.save_as.show();
                    }
                    Gui::open_external(&file);
                };
            }));

        self.preferences
            .connect_activate(clone!(@weak gui => move |_, _| {
                gui.dialogs.preferences.show();
            }));

        self.about
            .connect_activate(clone!(@strong gui => move |_, _| {
                gui.dialogs.about.show();
            }));

        self.quit.connect_activate(clone!(@weak gui => move |_, _| {
            gui.cleanup();
            gui.window.close();
        }));
    }
}

impl Convert for Gui {
    fn to_metric(&self) {
        self.adjustments.to_metric();
        self.bridge_spacing
            .set_value(self.bridge_spacing.value() * 20.4);
        self.nut_width.set_value(self.nut_width.value() * 20.4);
        self.scale.set_value(self.scale.value() * 20.4);
        self.scale_multi_fine
            .set_value(self.scale_multi_fine.value() * 20.4);
        self.bridge_spacing.set_digits(2);
        self.nut_width.set_digits(2);
        self.scale_fine.set_digits(2);
        self.scale_multi_fine.set_digits(2);
    }

    fn to_imperial(&self) {
        self.adjustments.to_imperial();
        self.bridge_spacing
            .set_value(self.bridge_spacing.value() / 20.4);
        self.nut_width.set_value(self.nut_width.value() / 20.4);
        self.scale.set_value(self.scale.value() / 20.4);
        self.scale_multi_fine
            .set_value(self.scale_multi_fine.value() / 20.4);
        self.bridge_spacing.set_digits(3);
        self.nut_width.set_digits(3);
        self.scale_fine.set_digits(3);
        self.scale_multi_fine.set_digits(3);
    }
}

impl Gui {
    fn init() -> Self {
        let builder = gtk::Builder::from_string(include_str!("gui.ui"));
        let window: gtk::ApplicationWindow = builder.object("mainWindow").unwrap();

        Self {
            window: window.clone(),
            image_preview: builder.object("image_preview").unwrap(),
            scale: builder.object("scale_course").unwrap(),
            scale_fine: builder.object("scale_fine").unwrap(),
            variant: builder.object("combo_box_variant").unwrap(),
            handedness: builder.object("combo_box_handedness").unwrap(),
            scale_multi_course: builder.object("scale_multi_course").unwrap(),
            scale_multi_fine: builder.object("scale_multi_fine").unwrap(),
            fret_count: builder.object("fret_count").unwrap(),
            perpendicular_fret: builder.object("perpendicular_fret").unwrap(),
            pfret_label: builder.object("pfret_label").unwrap(),
            nut_width: builder.object("nut_width").unwrap(),
            bridge_spacing: builder.object("bridge_spacing").unwrap(),
            dialogs: Dialogs::init(&window, &builder),
            adjustments: Adjustments::init(&builder),
        }
    }

    fn add_actions(&self, app: &gtk::Application) -> Actions {
        let actions = Actions::init();

        app.set_accels_for_action("win.open_template", &["<primary>O"]);
        app.set_accels_for_action("win.save", &["<primary>S"]);
        app.set_accels_for_action("win.save_as", &["<primary><shift>S"]);
        app.set_accels_for_action("win.open_external", &["<primary>E"]);
        app.set_accels_for_action("win.preferences", &["<primary><shift>P"]);
        app.set_accels_for_action("win.about", &["<primary>A"]);
        app.set_accels_for_action("win.quit", &["<primary>Q"]);

        self.window.add_action(&actions.open_template);
        self.window.add_action(&actions.save);
        self.window.add_action(&actions.save_as);
        self.window.add_action(&actions.open_external);
        self.window.add_action(&actions.preferences);
        self.window.add_action(&actions.about);
        self.window.add_action(&actions.quit);
        actions
    }

    fn get_handedness(&self) -> Handedness {
        match self.handedness.active() {
            Some(1) => Handedness::Left,
            _ => Handedness::Right,
        }
    }

    fn get_variant(&self) -> Variant {
        match self.variant.active() {
            Some(1) => {
                let scale = self.scale_multi_course.value();
                let hand = self.get_handedness();
                Variant::Multiscale(scale, hand)
            }
            _ => Variant::Monoscale,
        }
    }

    /// Takes the data represented by our Gtk widgets and outputs a Specs struct
    /// which will be used by the backend to render the svg image.
    #[allow(clippy::cast_sign_loss)]
    fn get_specs(&self) -> Specs {
        Specs::init(
            self.scale.value(),
            self.fret_count.value_as_int() as u32,
            self.get_variant(),
            self.nut_width.value(),
            match CONFIG.try_lock().unwrap().units {
                Units::Metric => self.bridge_spacing.value() + 6.0,
                Units::Imperial => self.bridge_spacing.value() + (6.0 / 20.4),
            },
            self.perpendicular_fret.value(),
        )
    }

    /// Performs a full render of the svg image without saving to disk, and
    /// refreshes the image preview with the new data.
    fn draw_preview(&self, swap: bool) {
        let cfg = CONFIG.try_lock().unwrap().clone();
        let image = self.get_specs().create_document(Some(cfg)).to_string();
        let bytes = gtk::glib::Bytes::from_owned(image.into_bytes());
        let stream = MemoryInputStream::from_bytes(&bytes);
        let width = self.image_preview.size(gtk::Orientation::Horizontal);
        let pixbuf =
            Pixbuf::from_stream_at_scale(&stream, width, -1, true, Option::<&Cancellable>::None);
        self.image_preview.set_pixbuf(Some(&pixbuf.unwrap()));
        if swap {
            if let Ok(mut file) = FILE.try_lock() {
                file.unset_current();
                self.set_window_title(&file);
            }
        }
    }

    fn toggle_multi(&self) {
        let value = self.variant.active() == Some(1);
        self.scale_multi_course.set_sensitive(value);
        self.scale_multi_fine.set_sensitive(value);
        if value {
            self.handedness.show();
            self.scale_multi_course.show();
            self.scale_multi_fine.show();
            self.perpendicular_fret.show();
            self.pfret_label.show();
        } else {
            self.handedness.hide();
            self.scale_multi_course.hide();
            self.scale_multi_fine.hide();
            self.perpendicular_fret.hide();
            self.pfret_label.hide();
        }
    }

    /// Updates the title of the program window with the name of the output file.
    fn set_window_title(&self, file: &File) {
        if !file.saved() {
            self.window.set_title(Some(&format!(
                "Gfret - {} - <unsaved>",
                env!("CARGO_PKG_VERSION")
            )));
        } else if file.current() {
            if let Some(filename) = file.filename() {
                self.window.set_title(Some(&format!(
                    "Gfret - {} - {}",
                    env!("CARGO_PKG_VERSION"),
                    filename
                )));
            }
        } else if let Some(filename) = file.filename() {
            self.window.set_title(Some(&format!(
                "Gfret - {} - {}*",
                env!("CARGO_PKG_VERSION"),
                filename
            )));
        }
    }

    /// Sets widget state to match temmplate
    pub fn load_template(&self, template: &Template) {
        self.scale.set_value(template.scale);
        self.fret_count.set_value(template.count.into());
        match template.scale_treble {
            Some(s) => {
                self.scale_multi_course.set_value(s);
                self.variant.set_active(Some(1));
            }
            None => self.variant.set_active(Some(0)),
        };
        match template.handedness {
            Some(Handedness::Left) => self.handedness.set_active(Some(1)),
            _ => self.handedness.set_active(Some(0)),
        };
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
            scale_treble: match self.variant.active() {
                Some(1) => Some(self.scale_multi_course.value()),
                _ => None,
            },
            handedness: match self.variant.active() {
                Some(1) => match self.handedness.active() {
                    Some(1) => Some(Handedness::Left),
                    _ => Some(Handedness::Right),
                },
                _ => None,
            },
            nut: self.nut_width.value(),
            bridge: self.bridge_spacing.value(),
            pfret: Some(self.perpendicular_fret.value()),
        }
    }

    fn save(&self) {
        if let Ok(file) = FILE.try_lock() {
            if file.saved() {
                if let Some(filename) = file.filename() {
                    let cfg = CONFIG.try_lock().unwrap().clone();
                    let document = self.get_specs().create_document(Some(cfg));
                    let template = self.template_from_gui();
                    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
                    let name = filename.to_string();
                    // When this stabilizes:
                    // Mutex::unlock(file);
                    mem::drop(file);
                    thread::spawn(move || {
                        let mut file = FILE.try_lock().unwrap();
                        if let Err(e) = template.save_to_file(&PathBuf::from(&name)) {
                            eprintln!("Error saving template: {e}");
                        }
                        match file.do_save(&name, &document) {
                            Ok(_) => {
                                sender
                                    .send("File saved".to_string())
                                    .expect("Error sending message");
                            }
                            Err(e) => {
                                sender
                                    .send(format!("{}", e))
                                    .expect("Error sending message");
                            }
                        }
                    });
                    let window = self.window.clone();
                    receiver.attach(None, move |response| {
                        match response.as_str() {
                            "File saved" => {
                                println!("Output saved as {}", &filename);
                                window.set_title(Some(&format!(
                                    "Gfret - {} - {}",
                                    env!("CARGO_PKG_VERSION"),
                                    filename,
                                )));
                            }
                            _ => eprintln!("Error saving file"),
                        }
                        Continue(false)
                    });
                }
            } else {
                self.dialogs.save_as.show();
            }
        }
    }

    fn save_as(&self, res: ResponseType) {
        if res == ResponseType::Accept {
            if let Some(filename) = self.dialogs.get_save_path() {
                let cfg = CONFIG.try_lock().unwrap().clone();
                let document = self.get_specs().create_document(Some(cfg));
                let template = self.template_from_gui();
                let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
                let name = filename.to_string();
                thread::spawn(move || {
                    let mut file = FILE.try_lock().unwrap();
                    if let Err(e) = template.save_to_file(&PathBuf::from(&name)) {
                        eprintln!("Error saving template: {e}");
                    }
                    match file.do_save(&name, &document) {
                        Ok(_) => {
                            sender
                                .send("File saved".to_string())
                                .expect("Error sending message");
                        }
                        Err(e) => {
                            sender.send(format!("{e}")).expect("Error sending message");
                        }
                    }
                });
                let window = self.window.clone();
                receiver.attach(None, move |response| {
                    match response.as_str() {
                        "File saved" => {
                            println!("Output saved as {}", &filename);
                            window.set_title(Some(&format!(
                                "Gfret - {} - {}",
                                env!("CARGO_PKG_VERSION"),
                                filename,
                            )));
                        }
                        _ => eprintln!("Error saving file"),
                    }
                    Continue(false)
                });
            }
        }
    }

    fn open_external(file: &File) {
        if let Some(filename) = file.filename() {
            let cfg = Config::from_file().unwrap_or_default();
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
        if let Err(e) = data.save_statefile() {
            eprintln!("Error saving statefile: {e}");
        }
    }
}

pub fn run(template: Option<&str>) {
    let template = template.map(ToString::to_string);
    let application = gtk::Application::new(
        Some("org.hitchhiker-linux.gfret"),
        gtk::gio::ApplicationFlags::default(),
    );
    application.add_main_option(
        "template",
        Char::from(b't'),
        OptionFlags::NONE,
        OptionArg::String,
        "",
        None,
    );
    application.connect_activate(move |app| {
        let gui = build_ui(app);
        if let Some(template) = &template {
            match Template::load_from_file(PathBuf::from(template.clone())) {
                Ok(t) => gui.load_template(&t),
                Err(e) => eprintln!("Error loading template: {e}"),
            }
        }
    });
    application.run();
}

fn build_ui(application: &Application) -> Rc<Gui> {
    let gui = Rc::new(Gui::init());
    let cfg = CONFIG.try_lock().unwrap().clone();
    let units = cfg.units;
    if units == Units::Imperial {
        gui.adjustments.to_imperial();
    }
    let mut statefile = config::get_config_dir();
    statefile.push("state.toml");
    if statefile.exists() {
        match Template::load_from_file(statefile) {
            Ok(t) => gui.load_template(&t),
            Err(e) => eprintln!("Error loading statefile: {e}"),
        }
    }

    gui.add_actions(application).connect(&gui);

    gui.window.set_title(Some(&format!(
        "Gfret - {} - <unsaved>",
        env!("CARGO_PKG_VERSION")
    )));

    gui.window.set_application(Some(application));
    gui.toggle_multi();
    gui.draw_preview(false);

    gui.scale
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(false);
        }));

    gui.variant.connect_changed(clone!(@weak gui => move |_| {
        gui.toggle_multi();
        gui.draw_preview(true);
    }));

    gui.handedness
        .connect_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.scale_multi_course
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.fret_count
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.perpendicular_fret
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.nut_width
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.bridge_spacing
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.dialogs
        .save_as
        .connect_response(clone!(@weak gui => move |dlg,res| {
            gui.save_as(res);
            dlg.hide();
        }));

    gui.dialogs
        .open_template
        .connect_response(clone!(@weak gui => move |dlg,res| {
            if res == ResponseType::Accept {
                if let Some(path) = gui.dialogs.get_template_path() {
                    match Template::load_from_file(path) {
                        Ok(t) => gui.load_template(&t),
                        Err(e) => eprintln!("Error opening template: {e}"),
                    }
                }
            }
            dlg.hide();
        }));

    gui.dialogs
        .preferences
        .window()
        .connect_response(clone!(@weak gui => move |dlg,res| {
            if res == ResponseType::Accept {
                let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
                let newcfg = gui.dialogs.preferences.config_from_widgets();
                let cfg = newcfg.clone();
                thread::spawn(move || {
                    match cfg.save_to_file(&crate::config::get_config_file()) {
                        Ok(_) => sender.send("success".to_string()),
                        Err(e) => sender.send(format!("{e}")),
                    }
                });
                receiver.attach(None, move |response| {
                    match response.as_str() {
                        "success" => {
                            let units = CONFIG.try_lock().unwrap().units;
                            if units != newcfg.units {
                                match newcfg.units {
                                    Units::Metric => gui.to_metric(),
                                    Units::Imperial => gui.to_imperial(),
                                }
                            }
                            {   let mut cfg = CONFIG.lock().unwrap();
                                *cfg = newcfg.truncate(); }
                            gui.draw_preview(true);
                        },
                        e => eprintln!("{e}"),
                    }
                    Continue(false)
                });
            }
            dlg.hide();
        }));

    gui.window.show();
    gui
}
