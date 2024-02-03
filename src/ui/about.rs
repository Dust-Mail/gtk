use gtk::License;

use crate::constants::{AUTHORS, DESCRIPTION, HOME_PAGE, LICENSE, NAME, REPO_URL, VERSION};

pub fn create_about_window() -> adw::AboutWindow {
    adw::AboutWindow::builder()
        .application_name(NAME)
        .comments(DESCRIPTION)
        .developer_name("The Dust-Mail project")
        .developers(AUTHORS.split(";").collect::<Vec<&str>>())
        .issue_url(format!("{}/issues", REPO_URL))
        .website(HOME_PAGE)
        .license(LICENSE)
        .license_type(License::Custom)
        .version(VERSION)
        .build()
}
