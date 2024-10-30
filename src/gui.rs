use gtk::{
    gdk, glib, prelude::*, Application, ApplicationWindow, Box, Button, ColorButton, DropDown,
    Entry, HeaderBar, Image, Label, MessageDialog, Orientation, Popover, SpinButton, Stack,
    StackSidebar, StringList, Switch,
};

use hyprparser::HyprlandConfig;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

#[derive(Debug)]
pub enum WidgetType {
    Switch(Switch),
    SpinButton(SpinButton),
    Entry(Entry),
    ColorButton(ColorButton),
    DropDown(DropDown),
}

pub struct ConfigGUI {
    pub window: ApplicationWindow,
    pub config_widgets: HashMap<String, WidgetType>,
    pub save_button: Button,
    pub content_box: Box,
    pub changed_options: Rc<RefCell<HashMap<(String, String), String>>>,
    pub stack: Stack,
    pub sidebar: StackSidebar,
    pub load_config_button: Button,
    pub save_config_button: Button,
    pub gear_menu: Rc<RefCell<Popover>>,
}

impl ConfigGUI {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1000)
            .default_height(600)
            .build();

        let header_bar = HeaderBar::builder()
            .show_title_buttons(false)
            .title_widget(&gtk::Label::new(Some("Hyprland Configuration")))
            .build();

        let gear_button = Button::from_icon_name("emblem-system-symbolic");
        header_bar.pack_start(&gear_button);

        let gear_menu = Rc::new(RefCell::new(Popover::new()));
        gear_menu.borrow().set_parent(&gear_button);

        let gear_menu_box = Box::new(Orientation::Vertical, 5);
        gear_menu_box.set_margin_top(5);
        gear_menu_box.set_margin_bottom(5);
        gear_menu_box.set_margin_start(5);
        gear_menu_box.set_margin_end(5);

        let save_config_button = Button::with_label("Save HyprGUI Config");
        let load_config_button = Button::with_label("Load HyprGUI Config");

        gear_menu_box.append(&load_config_button);
        gear_menu_box.append(&save_config_button);

        gear_menu.borrow().set_child(Some(&gear_menu_box));

        let gear_menu_clone = gear_menu.clone();
        gear_button.connect_clicked(move |_| {
            gear_menu_clone.borrow().popup();
        });

        let tooltip_button = Button::new();
        let question_mark_icon = Image::from_icon_name("dialog-question-symbolic");
        tooltip_button.set_child(Some(&question_mark_icon));
        tooltip_button.set_has_frame(false);
        header_bar.pack_start(&tooltip_button);

        let popover = Popover::new();
        let tooltip_text = "The save button saves the options that you chose in the gui and exports it to json format, likewise the load button loads these saved options from the exported json file; automatically filling in the options in the gui with the specified ones in the json file, clicking save to apply these changes is still necessary though.";
        let tooltip_label = Label::new(Some(tooltip_text));
        tooltip_label.set_margin_top(5);
        tooltip_label.set_margin_bottom(5);
        tooltip_label.set_margin_start(5);
        tooltip_label.set_margin_end(5);
        tooltip_label.set_wrap(true);
        tooltip_label.set_max_width_chars(50);
        popover.set_child(Some(&tooltip_label));

        tooltip_button.connect_clicked(move |button| {
            popover.set_parent(button);
            popover.popup();
        });

        let save_button = Button::with_label("Save");
        header_bar.pack_end(&save_button);

        window.set_titlebar(Some(&header_bar));

        let main_box = Box::new(Orientation::Vertical, 0);

        let content_box = Box::new(Orientation::Horizontal, 0);
        main_box.append(&content_box);

        window.set_child(Some(&main_box));

        let stack = Stack::new();
        let sidebar = StackSidebar::new();
        sidebar.set_stack(&stack);

        content_box.append(&sidebar);
        content_box.append(&stack);

        let mut config_widgets = HashMap::new();
        let descriptions = get_hyprctl_descriptions();

        if let Some(items) = descriptions.as_array() {
            for item in items {
                if let (Some(value), Some(description), Some(data_type)) = (
                    item.get("value").and_then(|v| v.as_str()),
                    item.get("description").and_then(|v| v.as_str()),
                    item.get("type").and_then(|v| v.as_i64()),
                ) {
                    let (category, name) = match value.split_once(':') {
                        Some((cat, name)) => (cat, name),
                        None => continue,
                    };

                    let category_box = if stack.child_by_name(category).is_none() {
                        let box_ = Box::new(Orientation::Vertical, 10);
                        box_.set_margin_start(10);
                        box_.set_margin_end(10);
                        box_.set_margin_top(10);
                        box_.set_margin_bottom(10);
                        stack.add_titled(&box_, Some(category), category);
                        box_
                    } else {
                        stack
                            .child_by_name(category)
                            .unwrap()
                            .downcast::<Box>()
                            .unwrap()
                    };

                    let (min, max, _step) = get_option_limits(name, description);

                    match data_type {
                        0 => Self::add_bool_option(
                            &category_box,
                            &mut config_widgets,
                            name,
                            name,
                            description,
                        ),
                        1..=3 => {
                            if description.contains("[0.0 - 1.0]") || name.contains("opacity") {
                                Self::add_int_option(
                                    &category_box,
                                    &mut config_widgets,
                                    name,
                                    name,
                                    description,
                                    min,
                                    max,
                                )
                            } else {
                                Self::add_string_option(
                                    &category_box,
                                    &mut config_widgets,
                                    name,
                                    name,
                                    description,
                                )
                            }
                        }
                        4 | 6 => {
                            let items = if let Some(data) = item.get("data") {
                                if let Some(value_str) = data.get("value").and_then(|v| v.as_str())
                                {
                                    value_str.split(',').collect::<Vec<_>>()
                                } else {
                                    vec![]
                                }
                            } else {
                                vec![]
                            };

                            if !items.is_empty() {
                                Self::add_dropdown_option(
                                    &category_box,
                                    &mut config_widgets,
                                    name,
                                    name,
                                    description,
                                    &items,
                                );
                            }
                        }
                        5 | 7 => Self::add_color_option(
                            &category_box,
                            &mut config_widgets,
                            name,
                            name,
                            description,
                        ),
                        _ => {}
                    }
                }
            }
        }

        ConfigGUI {
            window,
            config_widgets,
            save_button,
            content_box,
            changed_options: Rc::new(RefCell::new(HashMap::new())),
            stack,
            sidebar,
            load_config_button,
            save_config_button,
            gear_menu,
        }
    }

    pub fn setup_config_buttons(gui: Rc<RefCell<ConfigGUI>>) {
        let gui_clone = Rc::clone(&gui);
        gui.borrow().load_config_button.connect_clicked(move |_| {
            let gui = Rc::clone(&gui_clone);
            glib::MainContext::default().spawn_local(async move {
                let file_chooser = gtk::FileChooserDialog::new(
                    Some("Load HyprGUI Config"),
                    Some(&gui.borrow().window),
                    gtk::FileChooserAction::Open,
                    &[
                        ("Cancel", gtk::ResponseType::Cancel),
                        ("Open", gtk::ResponseType::Accept),
                    ],
                );

                if file_chooser.run_future().await == gtk::ResponseType::Accept {
                    if let Some(file) = file_chooser.file() {
                        if let Some(path) = file.path() {
                            gui.borrow_mut().load_hyprgui_config(&path);
                        }
                    }
                }
                file_chooser.close();
            });
        });

        let gui_clone = Rc::clone(&gui);
        gui.borrow().save_config_button.connect_clicked(move |_| {
            let gui = Rc::clone(&gui_clone);
            glib::MainContext::default().spawn_local(async move {
                let file_chooser = gtk::FileChooserDialog::new(
                    Some("Save HyprGUI Config"),
                    Some(&gui.borrow().window),
                    gtk::FileChooserAction::Save,
                    &[
                        ("Cancel", gtk::ResponseType::Cancel),
                        ("Save", gtk::ResponseType::Accept),
                    ],
                );

                file_chooser.set_current_name("hyprgui_config.json");

                if file_chooser.run_future().await == gtk::ResponseType::Accept {
                    if let Some(file) = file_chooser.file() {
                        if let Some(path) = file.path() {
                            gui.borrow_mut().save_hyprgui_config(&path);
                        }
                    }
                }
                file_chooser.close();
            });
        });
    }

    pub fn load_config(&mut self, config: &HyprlandConfig) {
        for category in self.get_sidebar_categories() {
            for line in config.to_string().lines() {
                let line = line.trim();
                if let Some((name, value)) = line.split_once('=') {
                    let name = name.trim();
                    let value = value.trim();
                    if let Some(widget) = self.config_widgets.get(name) {
                        self.set_widget_value(widget, value.to_string());
                        self.changed_options
                            .borrow_mut()
                            .insert((category.clone(), name.to_string()), value.to_string());
                    }
                }
            }
        }
    }

    pub fn get_changes(&self) -> &RefCell<HashMap<(String, String), String>> {
        &self.changed_options
    }

    fn load_hyprgui_config(&mut self, path: &PathBuf) {
        match fs::read_to_string(path) {
            Ok(content) => {
                if let Ok(config) = serde_json::from_str::<HashMap<String, String>>(&content) {
                    for (key, value) in config {
                        let parts: Vec<&str> = key.split(':').collect();
                        if parts.len() >= 2 {
                            let category = parts[0].to_string();
                            let name = parts[1..].join(":");
                            if let Some(widget) = self.config_widgets.get(&name) {
                                self.set_widget_value(widget, value.clone());
                                self.changed_options
                                    .borrow_mut()
                                    .insert((category, name), value);
                            }
                        }
                    }
                    self.custom_info_popup(
                        "Config Loaded",
                        "HyprGUI configuration loaded successfully.",
                        false,
                    );
                }
            }
            Err(e) => {
                self.custom_error_popup(
                    "Loading Failed",
                    &format!("Failed to read the configuration file: {}", e),
                    false,
                );
            }
        }
    }

    fn save_hyprgui_config(&mut self, path: &PathBuf) {
        let config: HashMap<String, String> = self
            .changed_options
            .borrow()
            .iter()
            .map(|((category, name), value)| (format!("{}:{}", category, name), value.clone()))
            .collect();

        match serde_json::to_string_pretty(&config) {
            Ok(json) => match fs::write(path, json) {
                Ok(_) => {
                    self.custom_info_popup(
                        "Config Saved",
                        "HyprGUI configuration saved successfully.",
                        false,
                    );
                }
                Err(e) => {
                    self.custom_error_popup(
                        "Saving Failed",
                        &format!("Failed to write the configuration file: {}", e),
                        false,
                    );
                }
            },
            Err(e) => {
                self.custom_error_popup(
                    "Serialization Failed",
                    &format!("Failed to serialize the configuration: {}", e),
                    false,
                );
            }
        }
    }

    pub fn custom_info_popup(&mut self, title: &str, text: &str, modal: bool) {
        let dialog = MessageDialog::builder()
            .message_type(gtk::MessageType::Info)
            .buttons(gtk::ButtonsType::Ok)
            .title(title)
            .text(text)
            .modal(modal)
            .build();

        dialog.connect_response(|dialog, _| {
            dialog.close();
        });

        dialog.show();
    }

    pub fn custom_error_popup(&mut self, title: &str, text: &str, modal: bool) {
        let dialog = MessageDialog::builder()
            .message_type(gtk::MessageType::Error)
            .buttons(gtk::ButtonsType::Ok)
            .title(title)
            .text(text)
            .modal(modal)
            .build();

        dialog.connect_response(|dialog, _| {
            dialog.close();
        });

        dialog.show();
    }

    pub fn custom_error_popup_critical(&mut self, title: &str, text: &str, modal: bool) {
        let dialog = MessageDialog::builder()
            .message_type(gtk::MessageType::Error)
            .buttons(gtk::ButtonsType::Ok)
            .title(title)
            .text(text)
            .modal(modal)
            .build();

        dialog.connect_response(|_, _| {
            std::process::exit(1);
        });

        dialog.show();
    }

    fn set_widget_value(&self, widget: &WidgetType, value: String) {
        match widget {
            WidgetType::Switch(switch) => {
                switch.set_active(value == "true" || value == "1");
            }
            WidgetType::SpinButton(spin) => {
                if let Ok(val) = value.parse::<f64>() {
                    spin.set_value(val);
                }
            }
            WidgetType::Entry(entry) => {
                entry.set_text(&value);
            }
            WidgetType::ColorButton(color) => {
                if let Ok(rgba) = gdk::RGBA::parse(&value) {
                    color.set_rgba(&rgba);
                }
            }
            WidgetType::DropDown(dropdown) => {
                if let Ok(idx) = value.parse::<u32>() {
                    dropdown.set_selected(idx);
                }
            }
        }
    }

    pub fn apply_changes(&mut self, config: &mut HyprlandConfig) {
        if let Some(category) = self.get_current_category() {
            let changes = self.changed_options.borrow();
            for ((category_, name_), value) in changes.iter() {
                if category_ == &category {
                    let entry = format!("{} = {}", name_, value);
                    config.add_entry(category_, &entry);
                }
            }
        }
    }

    fn add_bool_option(
        container: &Box,
        widgets: &mut HashMap<String, WidgetType>,
        name: &str,
        label: &str,
        _description: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        let switch = Switch::new();
        let label_widget = Label::new(Some(label));

        hbox.append(&label_widget);
        hbox.append(&switch);
        container.append(&hbox);

        widgets.insert(name.to_string(), WidgetType::Switch(switch));
    }

    fn add_int_option(
        container: &Box,
        widgets: &mut HashMap<String, WidgetType>,
        name: &str,
        label: &str,
        _description: &str,
        min: f64,
        max: f64,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        let spinbutton = SpinButton::with_range(min, max, 1.0);
        let label_widget = Label::new(Some(label));

        hbox.append(&label_widget);
        hbox.append(&spinbutton);
        container.append(&hbox);

        widgets.insert(name.to_string(), WidgetType::SpinButton(spinbutton));
    }

    fn add_string_option(
        container: &Box,
        widgets: &mut HashMap<String, WidgetType>,
        name: &str,
        label: &str,
        _description: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        let entry = Entry::new();
        let label_widget = Label::new(Some(label));

        hbox.append(&label_widget);
        hbox.append(&entry);
        container.append(&hbox);

        widgets.insert(name.to_string(), WidgetType::Entry(entry));
    }

    fn add_color_option(
        container: &Box,
        widgets: &mut HashMap<String, WidgetType>,
        name: &str,
        label: &str,
        _description: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        let colorbutton = ColorButton::new();
        let label_widget = Label::new(Some(label));

        hbox.append(&label_widget);
        hbox.append(&colorbutton);
        container.append(&hbox);

        widgets.insert(name.to_string(), WidgetType::ColorButton(colorbutton));
    }

    pub fn get_current_category(&self) -> Option<String> {
        self.stack.visible_child_name().map(|name| name.to_string())
    }

    pub fn get_sidebar_categories(&self) -> Vec<String> {
        let mut categories = Vec::new();
        let pages = self.stack.pages();
        for i in 0..pages.n_items() {
            if let Some(item) = pages.item(i) {
                if let Some(page) = item.downcast_ref::<gtk::StackPage>() {
                    if let Some(title) = page.title() {
                        categories.push(title.to_string());
                    }
                }
            }
        }
        categories
    }

    pub fn update_layout(&self) {
        self.content_box.set_visible(true);
        self.sidebar.set_visible(true);
    }

    fn add_dropdown_option(
        container: &Box,
        widgets: &mut HashMap<String, WidgetType>,
        name: &str,
        label: &str,
        _description: &str,
        items: &[&str],
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        let label_widget = Label::new(Some(label));
        let string_list = StringList::new(items);
        let dropdown = DropDown::builder().model(&string_list).build();

        hbox.append(&label_widget);
        hbox.append(&dropdown);
        container.append(&hbox);

        widgets.insert(name.to_string(), WidgetType::DropDown(dropdown));
    }
}

fn get_option_limits(name: &str, description: &str) -> (f64, f64, f64) {
    match name {
        "border_size" => (0.0, 10.0, 1.0),
        "gaps_in" | "gaps_out" | "gaps_workspaces" => (0.0, 50.0, 1.0),
        "resize_corner" => (0.0, 4.0, 1.0),
        "rounding" => (0.0, 20.0, 1.0),
        "active_opacity" | "inactive_opacity" | "fullscreen_opacity" => (0.0, 1.0, 0.1),
        "shadow_range" => (0.0, 50.0, 1.0),
        "shadow_render_power" => (1.0, 4.0, 1.0),
        "shadow_scale" => (0.0, 1.0, 0.1),
        "dim_strength" | "dim_special" | "dim_around" => (0.0, 1.0, 0.1),
        "blur:size" => (1.0, 20.0, 1.0),
        "blur:passes" => (1.0, 10.0, 1.0),
        "blur:noise" => (0.0, 1.0, 0.01),
        "blur:contrast" => (0.0, 2.0, 0.1),
        "blur:brightness" => (0.0, 2.0, 0.1),
        "blur:vibrancy" | "blur:vibrancy_darkness" => (0.0, 1.0, 0.1),
        "blur:popups_ignorealpha" => (0.0, 1.0, 0.1),
        "sensitivity" => (-1.0, 1.0, 0.1),
        "scroll_button" => (0.0, 9.0, 1.0),
        "scroll_factor" => (0.1, 10.0, 0.1),
        "follow_mouse" => (0.0, 3.0, 1.0),
        "float_switch_override_focus" => (0.0, 2.0, 1.0),
        "workspace_swipe_fingers" => (2.0, 5.0, 1.0),
        "workspace_swipe_distance" => (100.0, 500.0, 10.0),
        "workspace_swipe_min_speed_to_force" => (0.0, 100.0, 1.0),
        "workspace_swipe_cancel_ratio" => (0.0, 1.0, 0.1),
        "workspace_swipe_direction_lock_threshold" => (0.0, 50.0, 1.0),
        "drag_into_group" => (0.0, 2.0, 1.0),
        "force_default_wallpaper" => (-1.0, 2.0, 1.0),
        "vrr" => (0.0, 2.0, 1.0),
        "render_ahead_safezone" => (0.0, 10.0, 1.0),
        "new_window_takes_over_fullscreen" => (0.0, 2.0, 1.0),
        "initial_workspace_tracking" => (0.0, 2.0, 1.0),
        "render_unfocused_fps" => (1.0, 60.0, 1.0),
        "scroll_event_delay" => (0.0, 1000.0, 10.0),
        "workspace_center_on" => (0.0, 1.0, 1.0),
        "focus_preferred_method" => (0.0, 1.0, 1.0),
        "force_introspection" => (0.0, 2.0, 1.0),
        "explicit_sync" | "explicit_sync_kms" => (0.0, 2.0, 1.0),
        "min_refresh_rate" => (1.0, 240.0, 1.0),
        "hotspot_padding" => (0.0, 10.0, 1.0),
        "inactive_timeout" => (0.0, 60.0, 1.0),
        "zoom_factor" => (1.0, 5.0, 0.1),
        "damage_tracking" => (0.0, 2.0, 1.0),
        "watchdog_timeout" => (0.0, 60.0, 1.0),
        "error_limit" => (1.0, 100.0, 1.0),
        "error_position" => (0.0, 1.0, 1.0),
        "repeat_rate" => (1.0, 100.0, 1.0),
        "repeat_delay" => (100.0, 2000.0, 100.0),
        "touchpad:scroll_factor" => (0.1, 10.0, 0.1),
        "tablet:transform" => (0.0, 7.0, 1.0),
        "off_window_axis_events" => (0.0, 3.0, 1.0),
        "emulate_discrete_scroll" => (0.0, 2.0, 1.0),
        "focus_on_close" => (0.0, 1.0, 1.0),
        "groupbar:font_size" => (6.0, 32.0, 1.0),
        "groupbar:height" => (10.0, 50.0, 1.0),
        "groupbar:priority" => (0.0, 10.0, 1.0),
        "manual_crash" => (0.0, 1.0, 1.0),
        _ => {
            if description.contains("[0.0 - 1.0]") {
                (0.0, 1.0, 0.1)
            } else if description.contains("[0/1]") {
                (0.0, 1.0, 1.0)
            } else if description.contains("[0/1/2]") {
                (0.0, 2.0, 1.0)
            } else if name.contains("opacity") || name.contains("ratio") {
                (0.0, 1.0, 0.1)
            } else {
                (0.0, 50.0, 1.0)
            }
        }
    }
}

fn get_hyprctl_descriptions() -> serde_json::Value {
    let output = Command::new("hyprctl")
        .args(["descriptions", "-j"])
        .output()
        .expect("Failed to execute hyprctl");

    let output_str = String::from_utf8_lossy(&output.stdout);

    match serde_json::from_str(&output_str) {
        Ok(value) => value,
        Err(_) => serde_json::json!([]),
    }
}
