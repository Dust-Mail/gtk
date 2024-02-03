mod id;

use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use dust_mail::client::{
    self, EmailClient, IncomingEmailProtocol, OutgoingEmailProtocol, ThreadableEmailClient,
};
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk_macros::spawn;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::error::Result;

use crate::session::id::Identifier;

pub static SESSIONS: Lazy<Mutex<HashMap<String, ThreadableEmailClient>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct EmailLoginConfig {
    incoming: IncomingEmailProtocol,
    outgoing: OutgoingEmailProtocol,
}

glib::wrapper! {
    pub struct SessionManager(ObjectSubclass<imp::SessionManager>);
}

impl SessionManager {
    pub fn login(&self, credentials: EmailLoginConfig) {
        let fut = clone!(@weak self as this => async move {
             let result = clone!(@weak this => @default-panic, async move {
                let id: Identifier = (&credentials).into();

                let id: String = id.hash()?;

                let client = client::create(credentials.incoming, credentials.outgoing).await.map_err(Rc::new)?;

                let client: ThreadableEmailClient = client.into();

                let mut session_lock = SESSIONS.lock().expect("Failed to lock sessions mutex");

                session_lock.insert(id.clone(), client);

                Result::Ok(id)
             });

             match result.await {
                Ok(id) => this.emit_by_name("ready", &[&id]),
                Err(err) => this.emit_by_name("error", &[&err]),
             }
        });

        spawn!(fut);
    }

    pub fn get<I: AsRef<str>>(&self, id: I) -> Option<Arc<RwLock<EmailClient>>> {
        let lock = SESSIONS.lock().expect("Failed to lock sessions mutex");

        let client = lock.get(id.as_ref())?;

        Some(Arc::clone(client.as_ref()))
    }
}

mod imp {

    use std::cell::RefCell;

    use gtk::glib::{self, Properties};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::SessionManager)]
    pub struct SessionManager {
        #[property(get, set)]
        name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SessionManager {
        const NAME: &'static str = "DustMailSessionManager";
        type Type = super::SessionManager;
    }

    #[glib::derived_properties]
    impl ObjectImpl for SessionManager {}
}
