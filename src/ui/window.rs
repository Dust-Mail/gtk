use glib::Object;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }
}

mod imp {
    use gtk::subclass::prelude::*;

    use adw::subclass::prelude::*;

    use glib::subclass::InitializingObject;

    use gtk::{glib, CompositeTemplate};

    use crate::ui::sidebar::Sidebar;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/guusvanmeerveld/dustmail/window.ui")]
    pub struct Window {
        #[template_child]
        pub sidebar: TemplateChild<Sidebar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "DustMailWindow";

        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {}

    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}
