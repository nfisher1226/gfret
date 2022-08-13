use {
    adw::{
        gtk::{
            self, gio,
            glib::{
                self, once_cell::sync::Lazy, subclass::InitializingObject, BindingFlags, ParamSpec,
                ParamSpecDouble, ParamSpecUInt, Value,
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
    // Mapping the values from the above controls to properties of the window
    pub variant: Cell<u32>,
    pub bass_scale: Cell<f64>,
    pub treble_scale: Cell<f64>,
    pub nut: Cell<f64>,
    pub bridge: Cell<f64>,
    pub pfret: Cell<f64>,
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
            Lazy::new(|| vec![
                ParamSpecUInt::builder("variant").build(),
                ParamSpecDouble::builder("bass-scale").build(),
                ParamSpecDouble::builder("treble-scale").build(),
                ParamSpecDouble::builder("nut-width").build(),
                ParamSpecDouble::builder("bridge-spacing").build(),
                ParamSpecDouble::builder("perpendicular-fret").build(),
                ParamSpecUInt::builder("fret-count").build(),
            ]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "variant" => {
                let input = value.get().expect("The value needs to be of type `u32`.");
                self.variant.replace(input);
            }
            "bass-scale" => {
                let input_scale = value.get().expect("The value needs to be of type `f64`.");
                self.bass_scale.replace(input_scale);
            }
            "treble-scale" => {
                let input_scale = value.get().expect("The value needs to be of type `f64`.");
                self.treble_scale.replace(input_scale);
            }
            "nut-width" => {
                let input = value.get().expect("The value needs to be of type `f64`.");
                self.nut.replace(input);
            }
            "bridge-spacing" => {
                let input = value.get().expect("The value needs to be of type `f64`.");
                self.bridge.replace(input);
            }
            "perpendicular-fret" => {
                let input = value.get().expect("The value needs to be of type `f64`.");
                self.pfret.replace(input);
            }
            "fret-count" => {
                let input = value.get().expect("The value needs to be of type `f64`.");
                self.count.replace(input);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "variant" => self.variant.get().to_value(),
            "bass-scale" => self.bass_scale.get().to_value(),
            "treble-scale" => self.treble_scale.get().to_value(),
            "nut-width" => self.nut.get().to_value(),
            "bridge-spacing" => self.bridge.get().to_value(),
            "perpendicular-fret" => self.pfret.get().to_value(),
            "fret-count" => self.count.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.bind_property("variant", &obj.imp().variant_list.get(), "selected")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        obj.bind_property("bass-scale", &self.scale.adjustment(), "value")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        obj.bind_property("treble-scale", &self.scale_multi.adjustment(), "value")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        obj.bind_property("nut-width", &self.nut_width.adjustment(), "value")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        obj.bind_property("bridge-spacing", &self.bridge_spacing.adjustment(), "value")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        obj.bind_property("perpendicular-fret", &self.perpendicular_fret.adjustment(), "value")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        obj.bind_property("fret-count", &self.fret_count.adjustment(), "value")
            .transform_to(|_,value| {
                let num = value.get::<u32>().expect("The property needs to be of type `u32`.");
                let num = f64::from(num);
                Some(num.to_value())
            })
            .transform_from(|_,value| {
                let num = value.get::<f64>().expect("the property needs to be of type `f64`.");
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
