use {
    adw::{
        gtk::{
            self, gio,
            glib::{
                self, once_cell::sync::Lazy, subclass::InitializingObject, BindingFlags, ParamSpec,
                ParamSpecUInt, Value,
            },
            subclass::prelude::*,
            CompositeTemplate,
        },
        prelude::*,
        subclass::prelude::*,
    },
    std::cell::Cell,
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
    // Mapping this value to a u32 so as to bind to a settings property
    pub count: Cell<u32>,
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
            Lazy::new(|| vec![ParamSpecUInt::builder("fret-count").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "fret-count" => {
                let input = value.get().expect("The value needs to be of type `f64`.");
                self.count.replace(input);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "fret-count" => self.count.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.bind_property("fret-count", &self.fret_count.adjustment(), "value")
            .transform_to(|_, value| {
                let num = value
                    .get::<u32>()
                    .expect("The property needs to be of type `u32`.");
                let num = f64::from(num);
                Some(num.to_value())
            })
            .transform_from(|_, value| {
                let num = value
                    .get::<f64>()
                    .expect("the property needs to be of type `f64`.");
                let num = num as u32;
                Some(num.to_value())
            })
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
    }
}

impl AdwApplicationWindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwWindowImpl for Window {}
impl WindowImpl for Window {}
impl WidgetImpl for Window {}
impl BoxImpl for Window {}
