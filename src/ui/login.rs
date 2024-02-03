use glib::Object;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct LoginWindow(ObjectSubclass<imp::LoginWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl LoginWindow {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_view_switcher(&self) {}
}

mod imp {
    use gtk::subclass::prelude::*;

    use adw::subclass::prelude::*;

    use adw::ViewSwitcher;

    use glib::subclass::InitializingObject;

    use gtk::{glib, CompositeTemplate};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/guusvanmeerveld/dustmail/login.ui")]
    pub struct LoginWindow {
        #[template_child]
        pub view_switcher: TemplateChild<ViewSwitcher>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LoginWindow {
        const NAME: &'static str = "DustMailLoginWindow";

        type Type = super::LoginWindow;
        type ParentType = adw::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LoginWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().setup_view_switcher();
        }
    }

    impl WidgetImpl for LoginWindow {}

    impl WindowImpl for LoginWindow {}

    impl AdwWindowImpl for LoginWindow {}
}
