use gtk::prelude::*;
use gtk::Application;
use hyprparser::parse_config;
use std::fs;
use std::{cell::RefCell, rc::Rc};

mod gui;

const CONFIG_PATH: &str = "~/.config/hypr/hyprland.conf";

fn main() {
    let app = Application::builder()
        .application_id("nnyyxxxx.hyprgui")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let gui = Rc::new(RefCell::new(gui::ConfigGUI::new(app)));

    let config_str =
        fs::read_to_string(CONFIG_PATH.replace("~", &std::env::var("HOME").unwrap())).unwrap();
    let parsed_config = parse_config(&config_str);
    gui.borrow_mut().load_config(&parsed_config);

    let gui_clone = gui.clone();
    gui.borrow().save_button.connect_clicked(move |_| {
        save_config_file(gui_clone.clone());
    });

    gui.borrow().window.present();
}

fn save_config_file(gui: Rc<RefCell<gui::ConfigGUI>>) {
    let gui_ref = gui.borrow();
    let path = CONFIG_PATH.replace("~", &std::env::var("HOME").unwrap());
    let config_str = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading configuration file: {}", e);
            return;
        }
    };

    let mut parsed_config = parse_config(&config_str);

    gui_ref.save_config(&mut parsed_config);

    let updated_config_str = parsed_config.to_string();

    match fs::write(&path, updated_config_str) {
        Ok(_) => println!("Configuration saved successfully to {}", path),
        Err(e) => eprintln!("Error saving configuration: {}", e),
    }
}
