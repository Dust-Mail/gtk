use glib::Object;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct Settings(ObjectSubclass<imp::Settings>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Settings {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }
}

mod imp {
    use gtk::subclass::prelude::*;

    use adw::subclass::prelude::*;

    use glib::subclass::InitializingObject;

    use gtk::{glib, CompositeTemplate};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/guusvanmeerveld/dustmail/settings.ui")]
    pub struct Settings {}

    #[glib::object_subclass]
    impl ObjectSubclass for Settings {
        const NAME: &'static str = "DustMailSettings";

        type Type = super::Settings;
        type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Settings {}

    impl WidgetImpl for Settings {}

    impl WindowImpl for Settings {}

    impl AdwWindowImpl for Settings {}

    impl PreferencesWindowImpl for Settings {}
}
