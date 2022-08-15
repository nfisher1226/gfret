use adw::{
    gtk::{
        self,
        glib::{self, subclass::InitializingObject},
        prelude::*,
        subclass::prelude::*,
        traits::WidgetExt,
        CompositeTemplate,
    },
    prelude::*,
    subclass::{
        prelude::*,
        window::AdwWindowImpl,
        preferences_window::PreferencesWindowImpl
    }, traits::ActionRowExt, glib::BindingFlags,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "preferences.ui")]
pub struct PreferencesWindow {
    #[template_child]
    pub border_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub weight_adjustment: TemplateChild<gtk::Adjustment>,
    #[template_child]
    pub external_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub border_width: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub line_weight: TemplateChild<gtk::SpinButton>,
    #[template_child]
    fretline_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub fretboard_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub draw_centerline: TemplateChild<gtk::Switch>,
    #[template_child]
    centerline_color_row: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub centerline_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    print_specs: TemplateChild<gtk::Switch>,
    #[template_child]
    font_row: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub font_chooser: TemplateChild<gtk::FontButton>,
    
}

#[glib::object_subclass]
impl ObjectSubclass for PreferencesWindow {
    const NAME: &'static str = "PreferencesWindow";
    type Type = super::PreferencesWindow;
    type ParentType = adw::PreferencesWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PreferencesWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        // Bind some properties
        self.draw_centerline.get()
            .bind_property("active", &self.centerline_color_row.get(), "visible")
            .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
            .build();
        self.print_specs.get()
            .bind_property("active", &self.font_row.get(), "visible")
            .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
            .build();
    }
}

impl PreferencesWindowImpl for PreferencesWindow {}
impl AdwWindowImpl for PreferencesWindow {}
impl WindowImpl for PreferencesWindow {}
impl WidgetImpl for PreferencesWindow {}

