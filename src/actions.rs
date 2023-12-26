use gtk::prelude::*;

use adw::Application;

use gtk::gio::ActionEntry;

pub fn setup_application_actions(app: &Application) {
    let action_settings = ActionEntry::builder("settings")
        .activate(|app: &Application, _, _| {
            let preferences_window = crate::ui::settings::Settings::new(app);

            preferences_window.present();
        })
        .build();

    let action_about = ActionEntry::builder("about")
        .activate(|_app: &Application, _, _| {
            let about_window = crate::ui::about::create_about_window();

            about_window.present();
        })
        .build();

    app.add_action_entries([action_settings, action_about]);
}
