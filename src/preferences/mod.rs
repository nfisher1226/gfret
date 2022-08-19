mod imp;

use adw::{
    gdk,
    gtk::{
        self,
        glib::{self, Object},
    },
    prelude::*,
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
        @extends adw::PreferencesWindow, adw::Window, gtk::Window,
            gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl PreferencesWindow {
    #[must_use]
    pub fn new(app: &crate::Application) -> Self {
        let obj: Self = Object::new(&[]).expect("Cannot create preferences window");
        obj.bind_properties(app);
        obj
    }

    fn bind_properties(&self, app: &crate::Application) {
        let settings = &app.imp().settings;
        let imp = self.imp();
        settings
            .bind("external-editor", &imp.external_row.get(), "text")
            .build();
        settings
            .bind("units", &imp.units_selector.get(), "selected")
            .mapping(|variant, _vtype| {
                let item = variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`");
                match item.as_str() {
                    "imperial" => Some(1.to_value()),
                    _ => Some(0.to_value()),
                }
            })
            .set_mapping(|value, _vtype| {
                let num = value
                    .get::<u32>()
                    .expect("The value needs to be of type `u32`");
                match num {
                    0 => Some("metric".to_variant()),
                    _ => Some("imperial".to_variant()),
                }
            })
            .build();
        settings
            .bind("border-width", &imp.border_adjustment.get(), "value")
            .build();
        settings
            .bind("line-weight", &imp.weight_adjustment.get(), "value")
            .build();
        settings
            .bind("fretline-color", &imp.fretline_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                let rgba = variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<gdk::RGBA>()
                    .expect("Cannot parse RGBA from string");
                Some(rgba.to_value())
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<gdk::RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.to_variant())
            })
            .build();
        settings
            .bind("fretboard-color", &imp.fretboard_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                let rgba = variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<gdk::RGBA>()
                    .expect("Cannot parse RGBA from string");
                Some(rgba.to_value())
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<gdk::RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.to_variant())
            })
            .build();
        settings
            .bind("draw-centerline", &imp.draw_centerline.get(), "active")
            .build();
        settings
            .bind("centerline-color", &imp.centerline_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                let rgba = variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<gdk::RGBA>()
                    .expect("Cannot parse RGBA from string");
                Some(rgba.to_value())
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<gdk::RGBA>()
                    .expect("The value needs to be of type `gdk::RGBA`")
                    .to_string();
                Some(color.to_variant())
            })
            .build();
        settings
            .bind("print-specs", &imp.print_specs.get(), "active")
            .build();
        settings
            .bind("specs-font", &imp.font_chooser.get(), "font")
            .build();
    }
}
