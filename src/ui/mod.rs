use gtk::prelude::*;

use gtk::Application;

mod window;

use window::Window;

pub fn build(app: &Application) {
    let window = Window::new(app);

    // Present window
    window.present();
}
