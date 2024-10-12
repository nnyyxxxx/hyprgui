use gtk::prelude::*;
use gtk::Application;
use hyprparser::parse_config;
use std::fs;
use std::{cell::RefCell, rc::Rc};

mod gui;

fn main() {
    let app = Application::builder()
        .application_id("nnyyxxxx.hyprgui")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let gui = Rc::new(RefCell::new(gui::ConfigGUI::new(app)));

    gui.borrow_mut().hide_config_options();

    let gui_clone = gui.clone();
    gui.borrow().open_button.connect_clicked(move |_| {
        open_config_file(gui_clone.clone());
    });

    let gui_clone = gui.clone();
    gui.borrow().save_button.connect_clicked(move |_| {
        save_config_file(gui_clone.clone());
    });

    gui.borrow().window.present();
}

fn open_config_file(gui: Rc<RefCell<gui::ConfigGUI>>) {
    println!("open_config_file function called");
    let gui_clone = gui.clone();
    gui.borrow().open_config_file(move |path| {
        println!("Callback executed with path: {}", path);
        let config_str = fs::read_to_string(&path).unwrap();
        let parsed_config = parse_config(&config_str);
        gui_clone.borrow_mut().load_config(&parsed_config);
        gui_clone.borrow_mut().show_config_options();
        gui_clone.borrow().set_opened_file_path(path);
    });
    println!("open_config_file function completed");
}

fn save_config_file(gui: Rc<RefCell<gui::ConfigGUI>>) {
    let gui_ref = gui.borrow();
    if let Some(path) = gui_ref.get_opened_file_path() {
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
    } else {
        eprintln!("No configuration file has been opened yet.");
    }
}
