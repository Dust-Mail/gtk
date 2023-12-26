use gtk::prelude::*;
use gtk::{gio, glib};

const APP_ID: &str = "dev.guusvanmeerveld.DustMail";

mod actions;
mod constants;
mod ui;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("composite_templates_1.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();

    actions::setup_application_actions(&app);

    app.connect_activate(ui::build);

    app.run()
}
