use gtk::{prelude::*, Application};
use hyprparser::parse_config;
use std::{cell::RefCell, env, fs, path::Path, path::PathBuf, rc::Rc};

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
    gui::ConfigGUI::setup_config_buttons(gui.clone());

    let config_path_full = get_config_path();

    if !config_path_full.exists() {
        gui.borrow_mut().custom_error_popup_critical(
            "File not found",
            &format!("File not found: ~/{}", CONFIG_PATH),
            true,
        );
    } else {
        let config_str = match fs::read_to_string(config_path_full) {
            Ok(s) => s,
            Err(e) => {
                gui.borrow_mut().custom_error_popup_critical(
                    "Reading failed",
                    &format!("Failed to read the configuration file: {}", e),
                    true,
                );
                String::new()
            }
        };
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
    let config_str = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            gui_ref.custom_error_popup_critical(
                "Reading failed",
                &format!("Failed to read the configuration file: {}", e),
                true,
            );
            String::new()
        }
    };

    let mut parsed_config = parse_config(&config_str);
    let changes = gui_ref.get_changes();

    if !changes.borrow().is_empty() {
        gui_ref.apply_changes(&mut parsed_config);

        let updated_config_str = parsed_config.to_string();

        match fs::write(&path, updated_config_str) {
            Ok(_) => println!("Configuration saved to: ~/{}", CONFIG_PATH),
            Err(e) => {
                gui_ref.custom_error_popup(
                    "Saving failed",
                    &format!("Failed to save the configuration: {}", e),
                    true,
                );
            }
        }
    } else {
        gui_ref.custom_info_popup("Saving failed", "No changes to save.", true);
    }
}

fn get_config_path() -> PathBuf {
    Path::new(&env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(CONFIG_PATH)
}
