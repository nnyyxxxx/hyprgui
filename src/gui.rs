use gtk::{
    gdk, glib, prelude::*, Application, ApplicationWindow, Box, Button, ColorButton, DropDown,
    Entry, HeaderBar, Image, Label, MessageDialog, Orientation, Popover, ScrolledWindow,
    SearchEntry, SpinButton, Stack, StackSidebar, StringList, Switch, Widget,
};

use hyprparser::HyprlandConfig;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::modules::ConfigWidget;

pub fn add_dropdown_option(
    container: &Box,
    options: &mut HashMap<String, Widget>,
    name: &str,
    label: &str,
    description: &str,
    items: &[&str],
) {
    let hbox = Box::new(Orientation::Horizontal, 10);
    hbox.set_margin_start(10);
    hbox.set_margin_end(10);
    hbox.set_margin_top(5);
    hbox.set_margin_bottom(5);

    let label_box = Box::new(Orientation::Horizontal, 5);
    label_box.set_hexpand(true);

    let label_widget = Label::new(Some(label));
    label_widget.set_halign(gtk::Align::Start);

    let tooltip_button = Button::new();
    let question_mark_icon = Image::from_icon_name("dialog-question-symbolic");
    tooltip_button.set_child(Some(&question_mark_icon));
    tooltip_button.set_has_frame(false);

    let popover = Popover::new();
    let description_label = Label::new(Some(description));
    description_label.set_margin_top(5);
    description_label.set_margin_bottom(5);
    description_label.set_margin_start(5);
    description_label.set_margin_end(5);
    popover.set_child(Some(&description_label));
    popover.set_position(gtk::PositionType::Right);

    tooltip_button.connect_clicked(move |button| {
        popover.set_parent(button);
        popover.popup();
    });

    label_box.append(&label_widget);
    label_box.append(&tooltip_button);

    let string_list = StringList::new(items);
    let dropdown = DropDown::new(Some(string_list), None::<gtk::Expression>);
    dropdown.set_halign(gtk::Align::End);
    dropdown.set_width_request(100);

    hbox.append(&label_box);
    hbox.append(&dropdown);

    container.append(&hbox);

    options.insert(name.to_string(), dropdown.upcast());
}

pub fn get_option_limits(name: &str, description: &str) -> (f64, f64, f64) {
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
            } else if description.contains("[1 - 4]") {
                (0.0, 4.0, 1.0)
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

pub struct ConfigGUI {
    pub window: ApplicationWindow,
    pub config_widgets: HashMap<String, ConfigWidget>,
    pub save_button: Button,
    pub search_entry: SearchEntry,
    content_box: Box,
    changed_options: Rc<RefCell<HashMap<(String, String), String>>>,
    stack: Stack,
    pub sidebar: StackSidebar,
    load_config_button: Button,
    save_config_button: Button,
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

        let search_button = Button::from_icon_name("system-search-symbolic");
        let search_entry = SearchEntry::new();
        search_entry.set_width_chars(25);

        let popover = gtk::Popover::new();
        popover.set_child(Some(&search_entry));
        popover.set_position(gtk::PositionType::Bottom);
        popover.set_parent(&search_button);

        let save_config_button = Button::with_label("Save HyprGUI Config");
        let load_config_button = Button::with_label("Load HyprGUI Config");

        gear_menu_box.append(&load_config_button);
        gear_menu_box.append(&save_config_button);

        gear_menu.borrow().set_child(Some(&gear_menu_box));

        let gear_menu_clone = gear_menu.clone();
        gear_button.connect_clicked(move |_| {
            gear_menu_clone.borrow().popup();
        });

        let popover_clone = popover.clone();
        let search_entry_clone = search_entry.clone();
        search_button.connect_clicked(move |_| {
            if !popover_clone.is_visible() {
                popover_clone.popup();
                search_entry_clone.grab_focus();
            }
        });

        let popover_clone = popover.clone();
        search_entry.connect_activate(move |_| {
            popover_clone.popdown();
        });

        let popover_clone = popover.clone();
        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                popover_clone.popdown();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        search_entry.add_controller(key_controller);

        header_bar.pack_start(&search_button);

        let save_button = Button::with_label("Save");
        header_bar.pack_end(&save_button);

        window.set_titlebar(Some(&header_bar));

        let main_box = Box::new(Orientation::Vertical, 0);

        let content_box = Box::new(Orientation::Horizontal, 0);
        main_box.append(&content_box);

        window.set_child(Some(&main_box));

        let config_widgets = HashMap::new();

        let stack = Stack::new();

        let sidebar = StackSidebar::new();
        sidebar.set_stack(&stack);
        sidebar.set_width_request(200);

        ConfigGUI {
            window,
            config_widgets,
            save_button,
            search_entry,
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
        gui.borrow()
            .load_config_button
            .connect_clicked(move |button| {
                if let Some(popover) = button.ancestor(gtk::Popover::static_type()) {
                    if let Some(popover) = popover.downcast_ref::<gtk::Popover>() {
                        popover.popdown();
                    }
                }

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
        gui.borrow()
            .save_config_button
            .connect_clicked(move |button| {
                if let Some(popover) = button.ancestor(gtk::Popover::static_type()) {
                    if let Some(popover) = popover.downcast_ref::<gtk::Popover>() {
                        popover.popdown();
                    }
                }

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

    fn load_hyprgui_config(&mut self, path: &PathBuf) {
        match fs::read_to_string(path) {
            Ok(content) => {
                if let Ok(config) = serde_json::from_str::<HashMap<String, String>>(&content) {
                    for (key, value) in config {
                        let parts: Vec<&str> = key.split(':').collect();
                        if parts.len() >= 2 {
                            let category = parts[0].to_string();
                            let name = parts[1..].join(":");
                            if let Some(widget) = self.config_widgets.get(&category) {
                                if let Some(option_widget) = widget.options.get(&name) {
                                    self.set_widget_value(option_widget, &value);
                                    self.changed_options
                                        .borrow_mut()
                                        .insert((category, name), value);
                                }
                            }
                        }
                    }
                    self.custom_info_popup(
                        "Config Loaded",
                        "HyprGUI configuration loaded successfully.",
                        false,
                    );
                } else {
                    self.custom_error_popup(
                        "Invalid Config",
                        "Failed to parse the configuration file.",
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

    fn set_widget_value(&self, widget: &Widget, value: &str) {
        if let Some(spin_button) = widget.downcast_ref::<SpinButton>() {
            if let Ok(float_value) = value.parse::<f64>() {
                spin_button.set_value(float_value);
            }
        } else if let Some(entry) = widget.downcast_ref::<Entry>() {
            entry.set_text(value);
        } else if let Some(switch) = widget.downcast_ref::<Switch>() {
            switch.set_active(value == "true");
        } else if let Some(color_button) = widget.downcast_ref::<ColorButton>() {
            let dummy_config = HyprlandConfig::new();
            if let Some((red, green, blue, alpha)) = dummy_config.parse_color(value) {
                color_button.set_rgba(&gdk::RGBA::new(red, green, blue, alpha));
            }
        } else if let Some(dropdown) = widget.downcast_ref::<DropDown>() {
            let model = dropdown.model().unwrap();
            for i in 0..model.n_items() {
                if let Some(item) = model.item(i) {
                    if let Some(string_object) = item.downcast_ref::<gtk::StringObject>() {
                        if string_object.string() == value {
                            dropdown.set_selected(i);
                            break;
                        }
                    }
                }
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

    pub fn load_config(&mut self, config: &HyprlandConfig) {
        self.config_widgets.clear();
        self.content_box.set_visible(true);

        while let Some(child) = self.stack.first_child() {
            self.stack.remove(&child);
        }

        while let Some(child) = self.content_box.first_child() {
            self.content_box.remove(&child);
        }

        self.sidebar = StackSidebar::new();
        self.sidebar.set_stack(&self.stack);
        self.sidebar.set_width_request(200);

        self.content_box.append(&self.sidebar);
        self.content_box.append(&self.stack);

        self.stack.connect_visible_child_notify(move |stack| {
            if let Some(child) = stack.visible_child() {
                if let Some(scrolled_window) = child.downcast_ref::<ScrolledWindow>() {
                    let adj = scrolled_window.vadjustment();
                    adj.set_value(adj.lower());
                }
            }
        });

        let categories = [
            ("General", "general"),
            ("Decoration", "decoration"),
            ("Animations", "animations"),
            ("Input", "input"),
            ("Gestures", "gestures"),
            ("Misc", "misc"),
            ("Binds", "binds"),
            ("Group", "group"),
            ("Layouts", "layouts"),
            ("XWayland", "xwayland"),
            ("OpenGL", "opengl"),
            ("Render", "render"),
            ("Cursor", "cursor"),
            ("Debug", "debug"),
        ];

        for (display_name, category) in &categories {
            let widget = ConfigWidget::new(category);
            self.stack
                .add_titled(&widget.scrolled_window, Some(category), display_name);
            self.config_widgets.insert(category.to_string(), widget);
        }

        for (_, category) in &categories {
            if let Some(widget) = self.config_widgets.get(*category) {
                widget.load_config(config, category, self.changed_options.clone());
            }
        }

        self.changed_options.borrow_mut().clear();
    }

    pub fn get_changes(&self) -> Rc<RefCell<HashMap<(String, String), String>>> {
        self.changed_options.clone()
    }

    pub fn apply_changes(&self, config: &mut HyprlandConfig) {
        let changes = self.changed_options.borrow();
        for (category, widget) in &self.config_widgets {
            for (name, widget) in &widget.options {
                if let Some(value) = changes.get(&(category.to_string(), name.to_string())) {
                    let formatted_value =
                        if let Some(color_button) = widget.downcast_ref::<ColorButton>() {
                            let rgba = color_button.rgba();
                            format!(
                                "rgba({:02X}{:02X}{:02X}{:02X})",
                                (rgba.red() * 255.0) as u8,
                                (rgba.green() * 255.0) as u8,
                                (rgba.blue() * 255.0) as u8,
                                (rgba.alpha() * 255.0) as u8
                            )
                        } else {
                            value.clone()
                        };

                    if !formatted_value.is_empty() {
                        if category == "layouts" {
                            let parts: Vec<&str> = name.split(':').collect();
                            if parts.len() == 2 {
                                config.add_entry(
                                    parts[0],
                                    &format!("{} = {}", parts[1], formatted_value),
                                );
                            }
                        } else if name.contains(':') {
                            let parts: Vec<&str> = name.split(':').collect();
                            if parts.len() == 2 {
                                config.add_entry(
                                    &format!("{}.{}", category, parts[0]),
                                    &format!("{} = {}", parts[1], formatted_value),
                                );
                            }
                        } else {
                            config.add_entry(category, &format!("{} = {}", name, formatted_value));
                        }
                    }
                }
            }
        }
    }
}
