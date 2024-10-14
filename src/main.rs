use gtk::prelude::*;
use gtk::Application;
use hyprparser::parse_config;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::{cell::RefCell, env, rc::Rc};

mod gui;

const CONFIG_PATH: &str = ".config/hypr/hyprland.conf";

fn main() {
    let app = Application::builder()
        .application_id("nnyyxxxx.hyprgui")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let gui = Rc::new(RefCell::new(gui::ConfigGUI::new(app)));

    let config_path_full = get_config_path();

    if !config_path_full.exists() {
        gui.borrow_mut()
            .file_not_found(format!("~/{}", CONFIG_PATH));
    } else {
        let config_str = fs::read_to_string(config_path_full).unwrap();
        let parsed_config = parse_config(&config_str);
        gui.borrow_mut().load_config(&parsed_config);

        let gui_clone = gui.clone();
        gui.borrow().save_button.connect_clicked(move |_| {
            save_config_file(gui_clone.clone());
        });
    }

    gui.borrow().window.present();
}

fn save_config_file(gui: Rc<RefCell<gui::ConfigGUI>>) {
    let mut gui_ref = gui.borrow_mut();
    let path = get_config_path();
    let config_str = fs::read_to_string(&path).expect("Failed to read configuration file");

    let mut parsed_config = parse_config(&config_str);
    let changes = gui_ref.get_changes();

    if !changes.borrow().is_empty() {
        gui_ref.apply_changes(&mut parsed_config);

        let updated_config_str = parsed_config.to_string();

        match fs::write(&path, updated_config_str) {
            Ok(_) => println!("Configuration saved to: ~/{}", CONFIG_PATH),
            Err(e) => {
                gui_ref.saving_failed(e);
            }
        }
    } else {
        println!("No changes to save.");
    }
}

fn get_config_path() -> PathBuf {
    Path::new(&env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(CONFIG_PATH)
}
