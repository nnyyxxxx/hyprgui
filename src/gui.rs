use gtk::glib;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, CheckButton, Entry, Label, Orientation,
    ScrolledWindow, Stack, StackSidebar, Widget,
};

use hyprland_parser::HyprlandConfig;
use std::collections::HashMap;

pub struct ConfigGUI {
    pub window: ApplicationWindow,
    config_widgets: HashMap<String, ConfigWidget>,
    pub open_button: Button,
    pub save_button: Button,
    content_box: Box,
}

impl ConfigGUI {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hyprland Configuration")
            .default_width(800)
            .default_height(600)
            .build();

        let main_box = Box::new(Orientation::Vertical, 0);
        let open_button = Button::with_label("Open Config");
        let save_button = Button::with_label("Save");
        save_button.set_visible(false);

        let button_box = Box::new(Orientation::Horizontal, 5);
        button_box.set_halign(gtk::Align::Center);
        button_box.append(&open_button);
        button_box.append(&save_button);

        main_box.append(&button_box);

        let content_box = Box::new(Orientation::Horizontal, 0);
        main_box.append(&content_box);

        window.set_child(Some(&main_box));

        let config_widgets = HashMap::new();

        ConfigGUI {
            window,
            config_widgets,
            open_button,
            save_button,
            content_box,
        }
    }

    pub fn hide_config_options(&mut self) {
        self.content_box.set_visible(false);
        self.save_button.set_visible(false);
    }

    pub fn show_config_options(&mut self) {
        self.content_box.set_visible(true);
        self.save_button.set_visible(true);
    }

    pub fn load_config(&mut self, config: &HyprlandConfig) {
        self.config_widgets.clear();
        self.content_box.set_visible(true);

        let stack = Stack::new();
        let sidebar = StackSidebar::new();
        sidebar.set_stack(&stack);

        self.content_box.append(&sidebar);

        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_child(Some(&stack));
        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);
        self.content_box.append(&scrolled_window);

        for category in &[
            "General",
            "Decoration",
            "Animations",
            "Input",
            "Gestures",
            "Misc",
            "Binds",
            "WindowRules",
            "Layouts",
        ] {
            let widget = ConfigWidget::new(category);
            stack.add_titled(&widget.container, Some(category), category);
            self.config_widgets.insert(category.to_string(), widget);
        }

        for (category, widget) in &self.config_widgets {
            widget.load_config(config, category);
        }

        self.open_button.set_visible(false);
    }

    pub fn save_config(&self) -> HyprlandConfig {
        let mut config = HyprlandConfig::new();
        for (category, widget) in &self.config_widgets {
            widget.save_config(&mut config, category);
        }
        config
    }

    pub fn open_config_file<F>(&self, callback: F)
    where
        F: Fn(String) + 'static,
    {
        println!("open_config_file method called");
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open Config File"),
            Some(&self.window),
            gtk::FileChooserAction::Open,
            &[
                ("Cancel", gtk::ResponseType::Cancel),
                ("Open", gtk::ResponseType::Accept),
            ],
        );

        println!("FileChooserDialog created");

        file_chooser.set_modal(true);

        file_chooser.connect_response(move |dialog, response| {
            println!("File chooser response: {:?}", response);
            if response == gtk::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        if let Some(path_str) = path.to_str() {
                            println!("File selected: {}", path_str);
                            callback(path_str.to_string());
                        }
                    }
                }
            }
            dialog.close();
        });

        println!("About to show file chooser");
        file_chooser.show();
        println!("File chooser show() called");

        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if file_chooser.is_visible() {
                glib::ControlFlow::Continue
            } else {
                glib::ControlFlow::Break
            }
        });
    }
}

pub struct ConfigWidget {
    container: Box,
    options: HashMap<String, Widget>,
}

impl ConfigWidget {
    fn new(category: &str) -> Self {
        let container = Box::new(Orientation::Vertical, 10);
        container.set_margin_start(20);
        container.set_margin_end(20);
        container.set_margin_top(20);
        container.set_margin_bottom(20);

        let mut options = HashMap::new();

        match category {
            "General" => {
                Self::add_int_option(&container, &mut options, "border_size", "Border Size");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "no_border_on_floating",
                    "No Border on Floating",
                );
                Self::add_int_option(&container, &mut options, "gaps_in", "Gaps In");
                Self::add_int_option(&container, &mut options, "gaps_out", "Gaps Out");
                Self::add_string_option(
                    &container,
                    &mut options,
                    "col.active_border",
                    "Active Border Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "col.inactive_border",
                    "Inactive Border Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "cursor_inactive_timeout",
                    "Cursor Inactive Timeout",
                );
                Self::add_string_option(&container, &mut options, "layout", "Layout");
            }
            "Decoration" => {
                Self::add_bool_option(&container, &mut options, "rounding", "Enable Rounding");
                Self::add_int_option(&container, &mut options, "active_opacity", "Active Opacity");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "inactive_opacity",
                    "Inactive Opacity",
                );
                Self::add_bool_option(&container, &mut options, "blur", "Enable Blur");
                Self::add_int_option(&container, &mut options, "blur_size", "Blur Size");
                Self::add_int_option(&container, &mut options, "blur_passes", "Blur Passes");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "drop_shadow",
                    "Enable Drop Shadow",
                );
            }
            "Animations" => {
                Self::add_bool_option(&container, &mut options, "enabled", "Enable Animations");
                Self::add_string_option(&container, &mut options, "bezier", "Bezier Curve");
                Self::add_string_option(&container, &mut options, "animation", "Animation");
            }
            "Input" => {
                Self::add_int_option(&container, &mut options, "kb_layout", "Keyboard Layout");
                Self::add_int_option(&container, &mut options, "kb_variant", "Keyboard Variant");
                Self::add_int_option(&container, &mut options, "kb_model", "Keyboard Model");
                Self::add_int_option(&container, &mut options, "kb_options", "Keyboard Options");
                Self::add_int_option(&container, &mut options, "kb_rules", "Keyboard Rules");
                Self::add_int_option(&container, &mut options, "follow_mouse", "Follow Mouse");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "float_switch_override_focus",
                    "Float Switch Override Focus",
                );
            }
            "Gestures" => {
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe",
                    "Enable Workspace Swipe",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_fingers",
                    "Workspace Swipe Fingers",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_distance",
                    "Workspace Swipe Distance",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_invert",
                    "Workspace Swipe Invert",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_min_speed_to_force",
                    "Workspace Swipe Min Speed to Force",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_cancel_ratio",
                    "Workspace Swipe Cancel Ratio",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_create_new",
                    "Workspace Swipe Create New",
                );
            }
            "Misc" => {
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_hyprland_logo",
                    "Disable Hyprland Logo",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_splash_rendering",
                    "Disable Splash Rendering",
                );
                Self::add_bool_option(&container, &mut options, "no_vfr", "No VFR");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "mouse_move_enables_dpms",
                    "Mouse Move Enables DPMS",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "always_follow_on_dnd",
                    "Always Follow on DND",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "layers_hog_keyboard_focus",
                    "Layers Hog Keyboard Focus",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "animate_manual_resizes",
                    "Animate Manual Resizes",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_autoreload",
                    "Disable Autoreload",
                );
            }
            "Binds" => {}
            "WindowRules" => {}
            "Layouts" => {}
            _ => {}
        }

        ConfigWidget { container, options }
    }

    fn add_int_option(
        container: &Box,
        options: &mut HashMap<String, Widget>,
        name: &str,
        label: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 5);
        let label = Label::new(Some(label));
        let entry = Entry::new();

        hbox.append(&label);
        hbox.append(&entry);
        container.append(&hbox);

        options.insert(name.to_string(), entry.upcast());
    }

    fn add_bool_option(
        container: &Box,
        options: &mut HashMap<String, Widget>,
        name: &str,
        label: &str,
    ) {
        let checkbox = CheckButton::with_label(label);
        container.append(&checkbox);
        options.insert(name.to_string(), checkbox.upcast());
    }

    fn add_string_option(
        container: &Box,
        options: &mut HashMap<String, Widget>,
        name: &str,
        label: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 5);
        let label = Label::new(Some(label));
        let entry = Entry::new();

        hbox.append(&label);
        hbox.append(&entry);
        container.append(&hbox);

        options.insert(name.to_string(), entry.upcast());
    }

    fn load_config(&self, config: &HyprlandConfig, category: &str) {
        for (name, widget) in &self.options {
            let value = self.extract_value(config, category, name);
            if let Some(entry_widget) = widget.downcast_ref::<Entry>() {
                entry_widget.set_text(&value);
            } else if let Some(checkbox) = widget.downcast_ref::<CheckButton>() {
                checkbox.set_active(value == "true");
            }
        }
    }

    fn extract_value(&self, config: &HyprlandConfig, _category: &str, name: &str) -> String {
        let config_str = config.to_string();
        for line in config_str.lines() {
            if line.trim().starts_with(&format!("{} = ", name)) {
                return line
                    .split('=')
                    .nth(1)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_default();
            }
        }
        String::new()
    }

    fn save_config(&self, config: &mut HyprlandConfig, category: &str) {
        for (name, widget) in &self.options {
            let value = if let Some(entry) = widget.downcast_ref::<Entry>() {
                entry.text().to_string()
            } else if let Some(checkbox) = widget.downcast_ref::<CheckButton>() {
                checkbox.is_active().to_string()
            } else {
                continue;
            };
            config.add_entry(category, &format!("{} = {}", name, value));
        }
    }
}
