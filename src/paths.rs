use std::path::PathBuf;

use gtk::glib;
use once_cell::sync::Lazy;

use crate::constants;

/// The data directory for this application
pub const DATA: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = glib::user_data_dir();

    path.push(constants::NAME);

    path
});
