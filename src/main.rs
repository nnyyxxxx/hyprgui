use gtk::{prelude::*, Application};

mod config_gui;

use config_gui::ConfigGUI;

const APP_ID: &str = "dev.adamperkowski.hyprlandgui";

fn main() -> gtk::glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let gui = ConfigGUI::new(app);
    gui.window.present();
    gui.open_button.clone().connect_clicked(move |_| {
        gui.open_config_file();
    });
}
