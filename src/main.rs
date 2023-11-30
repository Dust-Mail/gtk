use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "dev.guusvanmeerveld.DustMail";

mod ui;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(ui::build);

    app.run()
}
