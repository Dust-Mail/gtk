use std::path::PathBuf;

use gtk::glib;
use once_cell::sync::Lazy;

use crate::{constants, paths};

pub const DB_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = paths::DATA.clone();

    path.push(format!("{}.db", constants::NAME));

    path
});
