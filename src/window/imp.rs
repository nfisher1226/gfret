use {
    adw::{prelude::*, subclass::prelude::*},
    gtk::{
        glib::{self, subclass::InitializingObject},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
};

#[derive(CompositeTemplate, Default)]
#[template(file = "gfret_window.ui")]
pub struct GfretWindow {
}

#[glib::object_subclass]
impl ObjectSubclass for GfretWindow {
    const NAME: &'static str = "GfretWindow";
    type Type = super::GfretWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GfretWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl AdwApplicationWindowImpl for GfretWindow {}
impl ApplicationWindowImpl for GfretWindow {}
impl AdwWindowImpl for GfretWindow {}
impl WindowImpl for GfretWindow {}
impl WidgetImpl for GfretWindow {}
impl BoxImpl for GfretWindow {}

