use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};

pub fn build(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Present window
    window.present();
}
