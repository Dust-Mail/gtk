use std::{io, rc::Rc, result};

use gtk::glib;

#[derive(Clone, thiserror::Error, Debug, glib::Boxed)]
#[boxed_type(name = "DustMailError")]
pub enum Error {
    #[error("I/O operation error: {0}")]
    Io(#[from] Rc<io::Error>),
    #[error("Email client error: {0}")]
    Client(#[from] Rc<dust_mail::error::Error>),
}

pub type Result<T> = result::Result<T, Error>;
