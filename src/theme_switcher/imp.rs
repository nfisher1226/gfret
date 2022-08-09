use gtk::{
    glib::{self, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    traits::WidgetExt,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "theme_switcher.ui")]
pub struct ThemeSwitcher {
    #[template_child]
    pub system_button: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub light_button: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub dark_button: TemplateChild<gtk::CheckButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for ThemeSwitcher {
    const NAME: &'static str = "ThemeSwitcher";
    type Type = super::ThemeSwitcher;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ThemeSwitcher {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_layout_manager(Some(&gtk::BinLayout::new()));
        self.system_button.connect_active_notify(move |_| {
            adw::StyleManager::default().set_color_scheme(adw::ColorScheme::Default);
        });
        self.light_button.connect_active_notify(move |_| {
            adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceLight);
        });
        self.dark_button.connect_active_notify(move |_| {
            adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceDark);
        });
    }
}

impl WidgetImpl for ThemeSwitcher {}
