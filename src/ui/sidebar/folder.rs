use gtk::{
    gio,
    glib::{self, Object},
};

glib::wrapper! {
    pub struct FolderObject(ObjectSubclass<imp::FolderObject>);
}

impl FolderObject {
    pub fn new<N: Into<String>, I: Into<String>, C: IntoIterator<Item = FolderObject>>(
        name: N,
        id: I,
        children: C,
    ) -> Self {
        let mut model = gio::ListStore::new::<FolderObject>();

        model.extend(children);

        Object::builder()
            .property("name", name.into())
            .property("id", id.into())
            .property("children", Some(model))
            .build()
    }
}

mod imp {
    use std::cell::RefCell;

    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{
        gio,
        glib::{self, Properties},
    };

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::FolderObject)]
    pub struct FolderObject {
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        id: RefCell<String>,
        #[property(get, set)]
        children: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FolderObject {
        const NAME: &'static str = "DustMailFolderObject";
        type Type = super::FolderObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for FolderObject {}
}
