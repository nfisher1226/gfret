mod imp;

use adw::{gtk::{
    self,
    gio::{self, ApplicationFlags},
    glib::{self, Object},
    prelude::ToValue,
}, prelude::ObjectExt, traits::AdwApplicationExt, glib::BindingFlags};

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, adw::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    #[must_use]
    pub fn new() -> Self {
        let obj: Self = Object::new(&[
            ("application-id", &Some("org.hitchhiker_linux.gfret")),
            ("flags", &ApplicationFlags::HANDLES_OPEN),
            ("register-session", &true.to_value()),
        ])
        .expect("Cannot create Application");
        obj.bind_property("theme", &obj.style_manager(), "color-scheme")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        obj
    }
}
