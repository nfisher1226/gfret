use {
    adw::{
        gtk::{
            self, gio,
            glib::{self, once_cell::sync::Lazy, subclass::InitializingObject},
            subclass::prelude::*,
            CompositeTemplate,
        },
        prelude::*,
        subclass::prelude::*,
    },
    std::cell::{Cell, RefCell},
};

#[derive(CompositeTemplate, Default)]
#[template(file = "gfret_window.ui")]
pub struct Window {
    #[template_child]
    pub title: TemplateChild<adw::WindowTitle>,
    #[template_child]
    pub menu_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub variant_list: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub overlay: TemplateChild<adw::ToastOverlay>,
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
    pub file: RefCell<Option<gio::File>>,
    pub changed: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> =
            Lazy::new(|| vec![glib::ParamSpecBoolean::builder("changed").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        match pspec.name() {
            "changed" => {
                let input = value
                    .get()
                    .expect("The value needs to be of type `boolean`");
                self.changed.replace(input);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "changed" => self.changed.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl AdwApplicationWindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwWindowImpl for Window {}
impl WindowImpl for Window {}
impl WidgetImpl for Window {}
impl BoxImpl for Window {}
