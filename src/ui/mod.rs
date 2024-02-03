use adw::prelude::*;

pub mod about;
pub mod login;
pub mod settings;
mod sidebar;
mod window;

use window::Window;

pub fn build(app: &adw::Application) {
    let window = Window::new(app);

    window.present();
}
