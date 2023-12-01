use gtk::prelude::*;
use gtk::{gio, glib, Application};

const APP_ID: &str = "dev.guusvanmeerveld.DustMail";

mod ui;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("composite_templates_1.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(ui::build);

    app.run()
}
