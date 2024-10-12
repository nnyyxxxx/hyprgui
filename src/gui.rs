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

    pub fn save_config(&self, config: &mut HyprlandConfig) {
        for (category, widget) in &self.config_widgets {
            widget.save_config(config, category);
        }
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
                Self::add_int_option(
                    &container,
                    &mut options,
                    "gaps_workspaces",
                    "Gaps Workspaces",
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
                    "col.active_border",
                    "Active Border Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "col.nogroup_border",
                    "No Group Border Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "col.nogroup_border_active",
                    "No Group Border Active Color",
                );
                Self::add_string_option(&container, &mut options, "layout", "Layout");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "no_focus_fallback",
                    "No Focus Fallback",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "resize_on_border",
                    "Resize on Border",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "extend_border_grab_area",
                    "Extend Border Grab Area",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "hover_icon_on_border",
                    "Hover Icon on Border",
                );
                Self::add_bool_option(&container, &mut options, "allow_tearing", "Allow Tearing");
                Self::add_int_option(&container, &mut options, "resize_corner", "Resize Corner");
            }
            "Decoration" => {
                Self::add_int_option(&container, &mut options, "rounding", "Rounding");
                Self::add_float_option(
                    &container,
                    &mut options,
                    "active_opacity",
                    "Active Opacity",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "inactive_opacity",
                    "Inactive Opacity",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "fullscreen_opacity",
                    "Fullscreen Opacity",
                );
                Self::add_bool_option(&container, &mut options, "drop_shadow", "Drop Shadow");
                Self::add_int_option(&container, &mut options, "shadow_range", "Shadow Range");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "shadow_render_power",
                    "Shadow Render Power",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "shadow_ignore_window",
                    "Shadow Ignore Window",
                );
                Self::add_string_option(&container, &mut options, "col.shadow", "Shadow Color");
                Self::add_string_option(
                    &container,
                    &mut options,
                    "col.shadow_inactive",
                    "Inactive Shadow Color",
                );
                Self::add_string_option(&container, &mut options, "shadow_offset", "Shadow Offset");
                Self::add_float_option(&container, &mut options, "shadow_scale", "Shadow Scale");
                Self::add_bool_option(&container, &mut options, "dim_inactive", "Dim Inactive");
                Self::add_float_option(&container, &mut options, "dim_strength", "Dim Strength");
                Self::add_float_option(&container, &mut options, "dim_special", "Dim Special");
                Self::add_float_option(&container, &mut options, "dim_around", "Dim Around");
                Self::add_string_option(&container, &mut options, "screen_shader", "Screen Shader");

                Self::add_bool_option(&container, &mut options, "blur:enabled", "Blur Enabled");
                Self::add_int_option(&container, &mut options, "blur:size", "Blur Size");
                Self::add_int_option(&container, &mut options, "blur:passes", "Blur Passes");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "blur:ignore_opacity",
                    "Blur Ignore Opacity",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "blur:new_optimizations",
                    "Blur New Optimizations",
                );
                Self::add_bool_option(&container, &mut options, "blur:xray", "Blur XRay");
                Self::add_float_option(&container, &mut options, "blur:noise", "Blur Noise");
                Self::add_float_option(&container, &mut options, "blur:contrast", "Blur Contrast");
                Self::add_float_option(
                    &container,
                    &mut options,
                    "blur:brightness",
                    "Blur Brightness",
                );
                Self::add_float_option(&container, &mut options, "blur:vibrancy", "Blur Vibrancy");
                Self::add_float_option(
                    &container,
                    &mut options,
                    "blur:vibrancy_darkness",
                    "Blur Vibrancy Darkness",
                );
                Self::add_bool_option(&container, &mut options, "blur:special", "Blur Special");
                Self::add_bool_option(&container, &mut options, "blur:popups", "Blur Popups");
                Self::add_float_option(
                    &container,
                    &mut options,
                    "blur:popups_ignorealpha",
                    "Blur Popups Ignore Alpha",
                );
            }
            "Animations" => {
                Self::add_bool_option(&container, &mut options, "enabled", "Enable Animations");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "first_launch_animation",
                    "First Launch Animation",
                );
            }
            "Input" => {
                Self::add_string_option(&container, &mut options, "kb_model", "Keyboard Model");
                Self::add_string_option(&container, &mut options, "kb_layout", "Keyboard Layout");
                Self::add_string_option(&container, &mut options, "kb_variant", "Keyboard Variant");
                Self::add_string_option(&container, &mut options, "kb_options", "Keyboard Options");
                Self::add_string_option(&container, &mut options, "kb_rules", "Keyboard Rules");
                Self::add_string_option(&container, &mut options, "kb_file", "Keyboard File");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "numlock_by_default",
                    "Numlock by Default",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "resolve_binds_by_sym",
                    "Resolve Binds by Symbol",
                );
                Self::add_int_option(&container, &mut options, "repeat_rate", "Repeat Rate");
                Self::add_int_option(&container, &mut options, "repeat_delay", "Repeat Delay");
                Self::add_float_option(&container, &mut options, "sensitivity", "Sensitivity");
                Self::add_string_option(
                    &container,
                    &mut options,
                    "accel_profile",
                    "Acceleration Profile",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "force_no_accel",
                    "Force No Acceleration",
                );
                Self::add_bool_option(&container, &mut options, "left_handed", "Left Handed");
                Self::add_string_option(&container, &mut options, "scroll_points", "Scroll Points");
                Self::add_string_option(&container, &mut options, "scroll_method", "Scroll Method");
                Self::add_int_option(&container, &mut options, "scroll_button", "Scroll Button");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "scroll_button_lock",
                    "Scroll Button Lock",
                );
                Self::add_float_option(&container, &mut options, "scroll_factor", "Scroll Factor");
                Self::add_bool_option(&container, &mut options, "natural_scroll", "Natural Scroll");
                Self::add_int_option(&container, &mut options, "follow_mouse", "Follow Mouse");
                Self::add_int_option(&container, &mut options, "focus_on_close", "Focus on Close");
                Self::add_bool_option(&container, &mut options, "mouse_refocus", "Mouse Refocus");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "float_switch_override_focus",
                    "Float Switch Override Focus",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "special_fallthrough",
                    "Special Fallthrough",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "off_window_axis_events",
                    "Off Window Axis Events",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "emulate_discrete_scroll",
                    "Emulate Discrete Scroll",
                );

                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:disable_while_typing",
                    "Disable While Typing",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:natural_scroll",
                    "Natural Scroll",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "touchpad:scroll_factor",
                    "Scroll Factor",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:middle_button_emulation",
                    "Middle Button Emulation",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "touchpad:tap_button_map",
                    "Tap Button Map",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:clickfinger_behavior",
                    "Clickfinger Behavior",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:tap-to-click",
                    "Tap to Click",
                );
                Self::add_bool_option(&container, &mut options, "touchpad:drag_lock", "Drag Lock");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:tap-and-drag",
                    "Tap and Drag",
                );

                Self::add_int_option(
                    &container,
                    &mut options,
                    "touchdevice:transform",
                    "Transform",
                );
                Self::add_string_option(&container, &mut options, "touchdevice:output", "Output");
                Self::add_bool_option(&container, &mut options, "touchdevice:enabled", "Enabled");

                Self::add_int_option(&container, &mut options, "tablet:transform", "Transform");
                Self::add_string_option(&container, &mut options, "tablet:output", "Output");
                Self::add_string_option(
                    &container,
                    &mut options,
                    "tablet:region_position",
                    "Region Position",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "tablet:region_size",
                    "Region Size",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "tablet:relative_input",
                    "Relative Input",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "tablet:left_handed",
                    "Left Handed",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "tablet:active_area_size",
                    "Active Area Size",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "tablet:active_area_position",
                    "Active Area Position",
                );
            }
            "Gestures" => {
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe",
                    "Workspace Swipe",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_fingers",
                    "Workspace Swipe Fingers",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_min_fingers",
                    "Workspace Swipe Min Fingers",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_distance",
                    "Workspace Swipe Distance",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_touch",
                    "Workspace Swipe Touch",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_invert",
                    "Workspace Swipe Invert",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_touch_invert",
                    "Workspace Swipe Touch Invert",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_min_speed_to_force",
                    "Workspace Swipe Min Speed to Force",
                );
                Self::add_float_option(
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
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_direction_lock",
                    "Workspace Swipe Direction Lock",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_direction_lock_threshold",
                    "Workspace Swipe Direction Lock Threshold",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_forever",
                    "Workspace Swipe Forever",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_use_r",
                    "Workspace Swipe Use R",
                );
            }
            "Group" => {
                Self::add_bool_option(&container, &mut options, "auto_group", "Auto Group");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "insert_after_current",
                    "Insert After Current",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "focus_removed_window",
                    "Focus Removed Window",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "drag_into_group",
                    "Drag Into Group",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "merge_groups_on_drag",
                    "Merge Groups on Drag",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "merge_floated_into_tiled_on_groupbar",
                    "Merge Floated Into Tiled on Groupbar",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "col.border_active",
                    "Active Border Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "col.border_inactive",
                    "Inactive Border Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "col.border_locked_active",
                    "Locked Active Border Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "col.border_locked_inactive",
                    "Locked Inactive Border Color",
                );

                Self::add_bool_option(&container, &mut options, "groupbar:enabled", "Enabled");
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:font_family",
                    "Font Family",
                );
                Self::add_int_option(&container, &mut options, "groupbar:font_size", "Font Size");
                Self::add_bool_option(&container, &mut options, "groupbar:gradients", "Gradients");
                Self::add_int_option(&container, &mut options, "groupbar:height", "Height");
                Self::add_bool_option(&container, &mut options, "groupbar:stacked", "Stacked");
                Self::add_int_option(&container, &mut options, "groupbar:priority", "Priority");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:render_titles",
                    "Render Titles",
                );
                Self::add_bool_option(&container, &mut options, "groupbar:scrolling", "Scrolling");
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:text_color",
                    "Text Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:col.active",
                    "Active Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:col.inactive",
                    "Inactive Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:col.locked_active",
                    "Locked Active Color",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:col.locked_inactive",
                    "Locked Inactive Color",
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
                Self::add_string_option(&container, &mut options, "col.splash", "Splash Color");
                Self::add_string_option(&container, &mut options, "font_family", "Font Family");
                Self::add_string_option(
                    &container,
                    &mut options,
                    "splash_font_family",
                    "Splash Font Family",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "force_default_wallpaper",
                    "Force Default Wallpaper",
                );
                Self::add_bool_option(&container, &mut options, "vfr", "VFR");
                Self::add_int_option(&container, &mut options, "vrr", "VRR");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "mouse_move_enables_dpms",
                    "Mouse Move Enables DPMS",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "key_press_enables_dpms",
                    "Key Press Enables DPMS",
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
                    "animate_mouse_windowdragging",
                    "Animate Mouse Window Dragging",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_autoreload",
                    "Disable Autoreload",
                );
                Self::add_bool_option(&container, &mut options, "enable_swallow", "Enable Swallow");
                Self::add_string_option(&container, &mut options, "swallow_regex", "Swallow Regex");
                Self::add_string_option(
                    &container,
                    &mut options,
                    "swallow_exception_regex",
                    "Swallow Exception Regex",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "focus_on_activate",
                    "Focus on Activate",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "mouse_move_focuses_monitor",
                    "Mouse Move Focuses Monitor",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "render_ahead_of_time",
                    "Render Ahead of Time",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "render_ahead_safezone",
                    "Render Ahead Safezone",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "allow_session_lock_restore",
                    "Allow Session Lock Restore",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "background_color",
                    "Background Color",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "close_special_on_empty",
                    "Close Special on Empty",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "new_window_takes_over_fullscreen",
                    "New Window Takes Over Fullscreen",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "exit_window_retains_fullscreen",
                    "Exit Window Retains Fullscreen",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "initial_workspace_tracking",
                    "Initial Workspace Tracking",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "middle_click_paste",
                    "Middle Click Paste",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "render_unfocused_fps",
                    "Render Unfocused FPS",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_xdg_env_checks",
                    "Disable XDG Env Checks",
                );
            }
            "Binds" => {
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "pass_mouse_when_bound",
                    "Pass Mouse When Bound",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "scroll_event_delay",
                    "Scroll Event Delay",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_back_and_forth",
                    "Workspace Back and Forth",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "allow_workspace_cycles",
                    "Allow Workspace Cycles",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_center_on",
                    "Workspace Center On",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "focus_preferred_method",
                    "Focus Preferred Method",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "ignore_group_lock",
                    "Ignore Group Lock",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "movefocus_cycles_fullscreen",
                    "Movefocus Cycles Fullscreen",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_keybind_grabbing",
                    "Disable Keybind Grabbing",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "window_direction_monitor_fallback",
                    "Window Direction Monitor Fallback",
                );
            }
            "XWayland" => {
                Self::add_bool_option(&container, &mut options, "enabled", "Enabled");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "use_nearest_neighbor",
                    "Use Nearest Neighbor",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "force_zero_scaling",
                    "Force Zero Scaling",
                );
            }
            "OpenGL" => {
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "nvidia_anti_flicker",
                    "Nvidia Anti Flicker",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "force_introspection",
                    "Force Introspection",
                );
            }
            "Render" => {
                Self::add_int_option(&container, &mut options, "explicit_sync", "Explicit Sync");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "explicit_sync_kms",
                    "Explicit Sync KMS",
                );
                Self::add_bool_option(&container, &mut options, "direct_scanout", "Direct Scanout");
            }
            "Cursor" => {
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "sync_gsettings_theme",
                    "Sync GSettings Theme",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "no_hardware_cursors",
                    "No Hardware Cursors",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "no_break_fs_vrr",
                    "No Break FS VRR",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "min_refresh_rate",
                    "Min Refresh Rate",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "hotspot_padding",
                    "Hotspot Padding",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "inactive_timeout",
                    "Inactive Timeout",
                );
                Self::add_bool_option(&container, &mut options, "no_warps", "No Warps");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "persistent_warps",
                    "Persistent Warps",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "warp_on_change_workspace",
                    "Warp on Change Workspace",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "default_monitor",
                    "Default Monitor",
                );
                Self::add_float_option(&container, &mut options, "zoom_factor", "Zoom Factor");
                Self::add_bool_option(&container, &mut options, "zoom_rigid", "Zoom Rigid");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "enable_hyprcursor",
                    "Enable Hyprcursor",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "hide_on_key_press",
                    "Hide on Key Press",
                );
                Self::add_bool_option(&container, &mut options, "hide_on_touch", "Hide on Touch");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "allow_dumb_copy",
                    "Allow Dumb Copy",
                );
            }
            "Debug" => {
                Self::add_bool_option(&container, &mut options, "overlay", "Overlay");
                Self::add_bool_option(&container, &mut options, "damage_blink", "Damage Blink");
                Self::add_bool_option(&container, &mut options, "disable_logs", "Disable Logs");
                Self::add_bool_option(&container, &mut options, "disable_time", "Disable Time");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "damage_tracking",
                    "Damage Tracking",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "enable_stdout_logs",
                    "Enable Stdout Logs",
                );
                Self::add_int_option(&container, &mut options, "manual_crash", "Manual Crash");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "suppress_errors",
                    "Suppress Errors",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "watchdog_timeout",
                    "Watchdog Timeout",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_scale_checks",
                    "Disable Scale Checks",
                );
                Self::add_int_option(&container, &mut options, "error_limit", "Error Limit");
                Self::add_int_option(&container, &mut options, "error_position", "Error Position");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "colored_stdout_logs",
                    "Colored Stdout Logs",
                );
            }
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

            if !value.is_empty() {
                if name.contains(':') {
                    let parts: Vec<&str> = name.split(':').collect();
                    if parts.len() == 2 {
                        config.add_entry(
                            &format!("{}.{}", category, parts[0]),
                            &format!("{} = {}", parts[1], value),
                        );
                    }
                } else {
                    config.add_entry(category, &format!("{} = {}", name, value));
                }
            }
        }
    }

    fn add_float_option(
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
}
