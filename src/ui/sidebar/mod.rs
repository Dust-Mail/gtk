use adw::{ActionRow, ExpanderRow};

use gtk::glib::clone;

use adw::prelude::*;
use gtk::subclass::prelude::*;

use gtk::{gio, glib, NoSelection, Widget};

use self::folder::FolderObject;

mod folder;

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

        let test_folder2 = FolderObject::new(
            "Test",
            "test",
            vec![
                FolderObject::new(
                    "Hello",
                    "hello",
                    vec![
                        FolderObject::new("Hello", "hello", Vec::new()),
                        FolderObject::new("Hello", "hello", Vec::new()),
                    ],
                ),
                FolderObject::new("Hello", "hello", Vec::new()),
            ],
        );

        model.append(&test_folder2);

        self.imp().folders.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.folders()));

        self.imp().folder_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as sidebar => @default-panic, move |obj| {
                let folder_object: &FolderObject = obj.downcast_ref().expect("The object should be of type `FolderObject`.");

                let folder_item = sidebar.create_folder_item(folder_object);

                folder_item.upcast()
            }),
        );

        self.set_folder_list_visible(&self.folders());
    }

    fn set_folder_list_visible(&self, folders: &gio::ListStore) {
        self.imp().folder_list.set_visible(folders.n_items() > 0);
    }

    fn create_folder_item(&self, folder_object: &FolderObject) -> Widget {
        if let Some(children) = folder_object.children() {
            if children.n_items() > 0 {
                let expander_row = ExpanderRow::builder().expanded(false).build();

                for child in children.iter() {
                    let child_object: FolderObject =
                        child.expect("The object should be of type `FolderObject`.");

                    let child_widget = self.create_folder_item(&child_object);

                    expander_row.add_row(&child_widget);
                }

                folder_object
                    .bind_property("name", &expander_row, "title")
                    .bidirectional()
                    .sync_create()
                    .build();

                return expander_row.upcast();
            }
        }

        let action_row = ActionRow::builder().build();

        folder_object
            .bind_property("name", &action_row, "title")
            .bidirectional()
            .sync_create()
            .build();

        action_row.upcast()
    }
}

mod imp {
    use std::cell::RefCell;

    use gtk::subclass::prelude::*;

    use gtk::glib::subclass::InitializingObject;

    use gtk::{gio, glib, CompositeTemplate, ListBox, SearchEntry};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/guusvanmeerveld/dustmail/sidebar.ui")]
    pub struct Sidebar {
        #[template_child]
        pub search: TemplateChild<SearchEntry>,
        #[template_child]
        pub folder_list: TemplateChild<ListBox>,
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
        }
    }

    impl WidgetImpl for Sidebar {}

    impl BoxImpl for Sidebar {}
}
