mod imp;

use {
    crate::ConvertUnits,
    adw::{
        gdk,
        gtk::{
            self,
            glib::{self, clone, Object},
        },
        prelude::*,
        subclass::prelude::*,
    },
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
    pub fn new(app: &crate::Application, win: &crate::Window) -> Self {
        let obj: Self = Object::builder().property("transient-for", win).build();
        obj.bind_properties(app);
        obj.connect_signals(app, win);
        obj
    }

    fn connect_signals(&self, app: &crate::Application, win: &crate::Window) {
        self.imp().units_selector.connect_selected_notify(
            clone!(@weak app, @weak win, @weak self as pwin => move |combo| {
                match combo.selected() {
                    0 => {
                        pwin.to_metric();
                        win.to_metric();
                    },
                    1 => {
                        pwin.to_imperial();
                        win.to_imperial();
                    },
                    _ => unimplemented!(),
                }
            }),
        );
        self.imp()
            .external_button
            .connect_clicked(clone!(@weak self as pwin => move |_| {
                pwin.appchooser();
            }));
    }

    fn bind_adjustments(&self, app: &crate::Application) {
        let settings = &app.imp().settings;
        let imp = self.imp();
        settings
            .bind(
                "step-increment",
                &imp.border_adjustment.get(),
                "step-increment",
            )
            .build();
        settings
            .bind(
                "page-increment",
                &imp.border_adjustment.get(),
                "page-increment",
            )
            .build();
        settings
            .bind("border-lower", &imp.border_adjustment.get(), "lower")
            .build();
        settings
            .bind("border-upper", &imp.border_adjustment.get(), "upper")
            .build();
        settings
            .bind(
                "step-increment",
                &imp.weight_adjustment.get(),
                "step-increment",
            )
            .build();
        settings
            .bind(
                "page-increment",
                &imp.weight_adjustment.get(),
                "page-increment",
            )
            .build();
        settings
            .bind("weight-lower", &imp.weight_adjustment.get(), "lower")
            .build();
        settings
            .bind("weight-upper", &imp.weight_adjustment.get(), "upper")
            .build();
    }

    fn bind_properties(&self, app: &crate::Application) {
        self.bind_adjustments(app);
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

    fn appchooser(&self) {
        let chooser = gtk::AppChooserDialog::for_content_type(
            Some(self),
            gtk::DialogFlags::all(),
            "image/svg+xml",
        );
        chooser.connect_response(move |dlg, res| {
            if res == gtk::ResponseType::Ok {
                if let Some(app_info) = dlg.app_info() {
                    if let Some(text) = app_info.commandline().map(|cmd| {
                        String::from(cmd.to_str().unwrap())
                            .split_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_string()
                    }) {
                        dlg.transient_for()
                            .expect("The window should be transient for a PreferencesWindow")
                            .downcast::<PreferencesWindow>()
                            .expect("The window should be of type `gfret::PreferencesWindow`")
                            .imp()
                            .external_row
                            .set_text(&text);
                    }
                };
            };
            dlg.hide();
        });
        chooser.show();
    }
}

impl ConvertUnits for PreferencesWindow {
    fn to_metric(&self) {
        self.imp().border_width.set_digits(2);
        let adjustment = self.imp().border_width.adjustment();
        let value = adjustment.value();
        adjustment.set_upper(40.0);
        adjustment.set_lower(10.0);
        adjustment.set_step_increment(0.1);
        adjustment.set_page_increment(0.5);
        adjustment.set_value(value * 20.4);
        self.imp().line_weight.set_digits(2);
        let adjustment = self.imp().line_weight.adjustment();
        let value = adjustment.value();
        adjustment.set_upper(2.0);
        adjustment.set_step_increment(0.1);
        adjustment.set_page_increment(0.5);
        adjustment.set_value(value * 20.4);
    }

    fn to_imperial(&self) {
        self.imp().border_width.set_digits(3);
        let mut adjustment = self.imp().border_width.adjustment();
        let mut value = adjustment.value();
        adjustment.set_upper(40.0 / 20.4);
        adjustment.set_lower(10.0 / 20.4);
        adjustment.set_step_increment(0.01);
        adjustment.set_page_increment(0.1);
        adjustment.set_value(value / 20.4);
        self.imp().line_weight.set_digits(2);
        adjustment = self.imp().line_weight.adjustment();
        value = adjustment.value();
        adjustment.set_upper(0.098);
        adjustment.set_step_increment(0.01);
        adjustment.set_page_increment(0.1);
        adjustment.set_value(value / 20.4);
    }
}
