mod imp;

use adw::{
    gdk,
    gio::SettingsBindFlags,
    gtk::{
        self,
        glib::{self, Object},
        pango::FontDescription,
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
        settings
            .bind("external-editor", &self.imp().external_row.get(), "text")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind("units", &self.imp().units_selector.get(), "selected")
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
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind("border-width", &self.imp().border_adjustment.get(), "value")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind("line-weight", &self.imp().weight_adjustment.get(), "value")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind("fretline-color", &self.imp().fretline_color.get(), "rgba")
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
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind("fretboard-color", &self.imp().fretboard_color.get(), "rgba")
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
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind(
                "draw-centerline",
                &self.imp().draw_centerline.get(),
                "active",
            )
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind(
                "centerline-color",
                &self.imp().centerline_color.get(),
                "rgba",
            )
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
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind("print-specs", &self.imp().print_specs.get(), "active")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        settings
            .bind("specs-font", &self.imp().font_chooser.get(), "font-desc")
            .mapping(|variant, _vtype| {
                let font = variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`");
                let font_desc = FontDescription::from_string(&font);
                Some(font_desc.to_value())
            })
            .set_mapping(|value, _vtype| {
                let font = value
                    .get::<FontDescription>()
                    .expect("The value needs to be of type `FontDescription`")
                    .to_string();
                Some(font.to_variant())
            })
            .flags(SettingsBindFlags::DEFAULT)
            .build();
    }
}
