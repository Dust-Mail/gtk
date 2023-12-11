use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::glib::{self, Object};

use super::folder::FolderObject;

glib::wrapper! {
    pub struct FolderItem(ObjectSubclass<imp::FolderItem>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl FolderItem {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, folder_object: &FolderObject) {
        let content_label = self.imp().content_label.get();

        let mut bindings = self.imp().bindings.borrow_mut();

        let content_label_binding = folder_object
            .bind_property("name", &content_label, "label")
            .sync_create()
            .build();

        bindings.push(content_label_binding);

        // Bind `task_object.completed` to `task_row.content_label.attributes`
        // let content_label_binding = folder_object
        //     .bind_property("completed", &content_label, "attributes")
        //     .sync_create()
        //     .transform_to(|_, active| {
        //         let attribute_list = AttrList::new();
        //         if active {
        //             // If "active" is true, content of the label will be strikethrough
        //             let attribute = AttrInt::new_strikethrough(true);
        //             attribute_list.insert(attribute);
        //         }
        //         Some(attribute_list.to_value())
        //     })
        //     .build();
        // // Save binding
        // bindings.push(content_label_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}

mod imp {
    use std::cell::RefCell;

    use gtk::subclass::prelude::*;

    use gtk::glib::subclass::InitializingObject;
    use gtk::glib::Binding;

    use gtk::{glib, CompositeTemplate, Label};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/guusvanmeerveld/dustmail/folder_item.ui")]
    pub struct FolderItem {
        #[template_child]
        pub content_label: TemplateChild<Label>,
        pub bindings: RefCell<Vec<Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FolderItem {
        const NAME: &'static str = "DustMailFolderItem";

        type Type = super::FolderItem;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FolderItem {}

    impl WidgetImpl for FolderItem {}

    impl BoxImpl for FolderItem {}
}
