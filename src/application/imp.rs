use {
    crate::{Actions, Window, ConvertUnits, CONFIG},
    adw::{
        gtk::{self, gdk,
        glib::{self, once_cell::sync::Lazy, ParamSpec, ParamSpecEnum, Value},
        prelude::*, subclass::prelude::*},
        subclass::prelude::*,
    },
    fretboard_layout::Units,
    std::cell::RefCell,
};

pub struct Application {
    pub theme: RefCell<adw::ColorScheme>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            theme: RefCell::new(adw::ColorScheme::Default),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = adw::Application;
}

impl ObjectImpl for Application {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> =
            Lazy::new(|| vec![
                ParamSpecEnum::builder("theme", adw::ColorScheme::static_type()).build()
            ]);
        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        value: &Value,
        pspec: &ParamSpec,
    ) {
        match pspec.name() {
            "theme" => {
                let input_theme =
                    value.get().expect("The value needs to be of type `adw::ColorScheme`.");
                self.theme.replace(input_theme);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "theme" => self.theme.borrow().clone().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl ApplicationImpl for Application {
    fn activate(&self, app: &Self::Type) {
        let window = Window::new(app);
        Actions::default().add(&window, app);
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_str!("../style.css").as_bytes());
        gtk::StyleContext::add_provider_for_display(
            &gdk::Display::default().expect("Cannot get display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        {
            let cfg = CONFIG.try_lock().unwrap().clone();
            if cfg.units == Units::Imperial {
                window.to_imperial();
            }
        }
        window.draw_preview();
        window.present();
    }
}

impl AdwApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
