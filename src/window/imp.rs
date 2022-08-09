use {
    adw::{prelude::*, subclass::prelude::*},
    gtk::{
        gio,
        glib::{self, subclass::InitializingObject},
        subclass::prelude::*,
        CompositeTemplate,
    },
};

#[derive(CompositeTemplate, Default)]
#[template(file = "gfret_window.ui")]
pub struct GfretWindow {
    #[template_child]
    pub title: TemplateChild<adw::WindowTitle>,
    #[template_child]
    pub variant_box: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub image_preview: TemplateChild<gtk::Picture>,
    #[template_child]
    pub scale: TemplateChild<gtk::Scale>,
    #[template_child]
    pub scale_fine: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub scale_multi: TemplateChild<gtk::Scale>,
    #[template_child]
    pub scale_multi_fine: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub nut_width: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub bridge_spacing: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub pfret_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub perpendicular_fret: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub fret_count: TemplateChild<gtk::SpinButton>,
    pub file: Option<gio::File>,
    pub changed: bool,
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
        self.variant_box.set_active_id(Some("monoscale"));
    }
}

impl AdwApplicationWindowImpl for GfretWindow {}
impl ApplicationWindowImpl for GfretWindow {}
impl AdwWindowImpl for GfretWindow {}
impl WindowImpl for GfretWindow {}
impl WidgetImpl for GfretWindow {}
impl BoxImpl for GfretWindow {}

