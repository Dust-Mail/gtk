use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::{gio, glib, ListItem, NoSelection, SignalListItemFactory};

use self::folder::FolderObject;
use self::folder_item::FolderItem;

mod folder;
mod folder_item;

glib::wrapper! {
    pub struct Sidebar(ObjectSubclass<imp::Sidebar>)
        @extends  gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}

impl Sidebar {
    fn folders(&self) -> gio::ListStore {
        self.imp()
            .folders
            .borrow()
            .clone()
            .expect("Could not get current folders.")
    }

    fn setup_folders(&self) {
        let model = gio::ListStore::new::<FolderObject>();

        let test_folder = FolderObject::new("Hello", "hello");
        model.append(&test_folder);

        self.imp().folders.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.folders()));

        self.imp().folder_list.set_model(Some(&selection_model));
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let folder_item = FolderItem::new();

            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&folder_item));
        });

        factory.connect_bind(move |_, list_item| {
            let folder_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<FolderObject>()
                .expect("The item has to be an `FolderObject`.");

            let folder_item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<FolderItem>()
                .expect("The child has to be a `FolderItem`.");

            folder_item.bind(&folder_object);
        });

        factory.connect_unbind(move |_, list_item| {
            let folder_item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<FolderItem>()
                .expect("The child has to be a `FolderItem`.");

            folder_item.unbind();
        });

        self.imp().folder_list.set_factory(Some(&factory));
    }
}

mod imp {
    use std::cell::RefCell;

    use gtk::subclass::prelude::*;

    use gtk::glib::subclass::InitializingObject;

    use gtk::{gio, glib, CompositeTemplate, ListView, SearchEntry};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/guusvanmeerveld/dustmail/sidebar.ui")]
    pub struct Sidebar {
        #[template_child]
        pub search: TemplateChild<SearchEntry>,
        #[template_child]
        pub folder_list: TemplateChild<ListView>,
        pub folders: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Sidebar {
        const NAME: &'static str = "DustMailSidebar";

        type Type = super::Sidebar;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Sidebar {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_folders();
            obj.setup_factory();
        }
    }

    impl WidgetImpl for Sidebar {}

    impl BoxImpl for Sidebar {}
}
