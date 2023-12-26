use adw::prelude::*;

pub mod about;
mod sidebar;
mod window;

use window::Window;

pub fn build(app: &adw::Application) {
    let window = Window::new(app);

    window.present();
}
