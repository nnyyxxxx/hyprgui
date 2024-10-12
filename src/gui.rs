use gtk::gdk;
use gtk::prelude::*;
use gtk::Switch;
use gtk::{
    Application, ApplicationWindow, Box, Button, CheckButton, ColorButton, Entry, Frame, HeaderBar,
    Label, Orientation, ScrolledWindow, Stack, StackSidebar, Widget,
};

use hyprparser::HyprlandConfig;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct ConfigGUI {
    pub window: ApplicationWindow,
    config_widgets: HashMap<String, ConfigWidget>,
    pub save_button: Button,
    content_box: Box,
    changed_options: Rc<RefCell<HashSet<(String, String)>>>,
}

impl ConfigGUI {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            .build();

        let header_bar = HeaderBar::builder()
            .show_title_buttons(false)
            .title_widget(&gtk::Label::new(Some("Hyprland Configuration")))
            .build();

        let save_button = Button::with_label("Save");
        header_bar.pack_end(&save_button);

        window.set_titlebar(Some(&header_bar));

        let main_box = Box::new(Orientation::Vertical, 0);

        let content_box = Box::new(Orientation::Horizontal, 0);
        main_box.append(&content_box);

        window.set_child(Some(&main_box));

        let config_widgets = HashMap::new();

        ConfigGUI {
            window,
            config_widgets,
            save_button,
            content_box,
            changed_options: Rc::new(RefCell::new(HashSet::new())),
        }
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
            "general",
            "decoration",
            "animations",
            "input",
            "gestures",
            "misc",
            "binds",
            "windowrules",
            "layouts",
        ] {
            let widget = ConfigWidget::new(category);
            stack.add_titled(&widget.container, Some(category), category);
            self.config_widgets.insert(category.to_string(), widget);
        }

        for (category, widget) in &self.config_widgets {
            widget.load_config(config, category, self.changed_options.clone());
        }
    }

    pub fn get_changes(&self) -> Rc<RefCell<HashSet<(String, String)>>> {
        self.changed_options.clone()
    }

    pub fn apply_changes(&self, config: &mut HyprlandConfig) {
        let changes = self.changed_options.borrow();
        for (category, widget) in &self.config_widgets {
            widget.apply_changes(config, category, &changes);
        }
    }
}

pub struct ConfigWidget {
    container: Box,
    options: HashMap<String, Widget>,
}

impl ConfigWidget {
    fn new(category: &str) -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        container.set_margin_start(20);
        container.set_margin_end(20);
        container.set_margin_top(20);
        container.set_margin_bottom(20);

        let mut options = HashMap::new();

        match category {
            "general" => {
                Self::add_section(
                    &container,
                    "General Settings",
                    "Configure general behavior.",
                );
                Self::add_section(&container, "Gaps", "Change gaps in & out, workspaces.");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "gaps_in",
                    "Gaps In",
                    "Gaps between windows.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "gaps_out",
                    "Gaps Out",
                    "Gaps between windows and monitor edges.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "gaps_workspaces",
                    "Gaps Workspaces",
                    "Gaps between workspaces. Stacks with gaps_out.",
                );

                Self::add_section(&container, "Borders", "Size, resize, floating...");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "border_size",
                    "Border Size",
                    "Size of the border around windows.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "no_border_on_floating",
                    "Border on Floating",
                    "Enable borders for floating windows.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "resize_on_border",
                    "Resize on Border",
                    "Enables resizing windows by clicking and dragging on borders and gaps.",
                );
                Self::add_int_option(&container, &mut options, "extend_border_grab_area", "Extend Border Grab Area", "Extends the area around the border where you can click and drag on, only used when general:resize_on_border is on.");
                Self::add_bool_option(&container, &mut options, "hover_icon_on_border", "Hover Icon on Border", "Show a cursor icon when hovering over borders, only used when general:resize_on_border is on.");

                Self::add_section(&container, "Colors", "Change borders colors.");
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.inactive_border",
                    "Inactive Border Color",
                    "Border color for inactive windows.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.active_border",
                    "Active Border Color",
                    "Border color for the active window.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.nogroup_border",
                    "No Group Border Color",
                    "Inactive border color for window that cannot be added to a group.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.nogroup_border_active",
                    "No Group Border Active Color",
                    "Active border color for window that cannot be added to a group.",
                );
            }
            "decoration" => {
                Self::add_section(
                    &container,
                    "Window Decoration",
                    "Configure window appearance.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "rounding",
                    "Rounding",
                    "The rounding of the window corners.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "active_opacity",
                    "Active Opacity",
                    "The opacity of active windows.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "inactive_opacity",
                    "Inactive Opacity",
                    "The opacity of inactive windows.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "fullscreen_opacity",
                    "Fullscreen Opacity",
                    "The opacity of fullscreen windows.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "drop_shadow",
                    "Drop Shadow",
                    "Enables the drop shadow.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "shadow_range",
                    "Shadow Range",
                    "The range of the drop shadow.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "shadow_render_power",
                    "Shadow Render Power",
                    "The render power of the drop shadow.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "shadow_ignore_window",
                    "Shadow Ignore Window",
                    "Ignores the window when rendering the drop shadow.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.shadow",
                    "Shadow Color",
                    "The color of the drop shadow.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.shadow_inactive",
                    "Inactive Shadow Color",
                    "The color of the drop shadow for inactive windows.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "shadow_offset",
                    "Shadow Offset",
                    "The offset of the drop shadow.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "shadow_scale",
                    "Shadow Scale",
                    "The scale of the drop shadow.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "dim_inactive",
                    "Dim Inactive",
                    "Dims inactive windows.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "dim_strength",
                    "Dim Strength",
                    "The strength of the dim effect.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "dim_special",
                    "Dim Special",
                    "The dim effect for special windows.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "dim_around",
                    "Dim Around",
                    "The dim effect around the windows.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "screen_shader",
                    "Screen Shader",
                    "The screen shader.",
                );

                Self::add_bool_option(
                    &container,
                    &mut options,
                    "blur:enabled",
                    "Blur Enabled",
                    "Enables the blur effect.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "blur:size",
                    "Blur Size",
                    "The size of the blur effect.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "blur:passes",
                    "Blur Passes",
                    "The number of blur passes.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "blur:ignore_opacity",
                    "Blur Ignore Opacity",
                    "Ignores the opacity when applying the blur effect.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "blur:new_optimizations",
                    "Blur New Optimizations",
                    "Enables the new optimizations for the blur effect.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "blur:xray",
                    "Blur XRay",
                    "Enables the XRay effect for the blur.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "blur:noise",
                    "Blur Noise",
                    "The noise level for the blur effect.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "blur:contrast",
                    "Blur Contrast",
                    "The contrast level for the blur effect.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "blur:brightness",
                    "Blur Brightness",
                    "The brightness level for the blur effect.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "blur:vibrancy",
                    "Blur Vibrancy",
                    "The vibrancy level for the blur effect.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "blur:vibrancy_darkness",
                    "Blur Vibrancy Darkness",
                    "The darkness level for the vibrancy effect.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "blur:special",
                    "Blur Special",
                    "Enables the blur effect for special windows.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "blur:popups",
                    "Blur Popups",
                    "Enables the blur effect for popups.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "blur:popups_ignorealpha",
                    "Blur Popups Ignore Alpha",
                    "Ignores the alpha channel when applying the blur effect for popups.",
                );
            }
            "animations" => {
                Self::add_section(
                    &container,
                    "Animation Settings",
                    "Configure animation behavior.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "enabled",
                    "Enable Animations",
                    "Enables animations.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "first_launch_animation",
                    "First Launch Animation",
                    "Enables the first launch animation.",
                );
            }
            "input" => {
                Self::add_section(&container, "Input Settings", "Configure input devices.");
                Self::add_section(
                    &container,
                    "Keyboard Settings",
                    "Configure keyboard behavior.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "kb_model",
                    "Keyboard Model",
                    "The keyboard model.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "kb_layout",
                    "Keyboard Layout",
                    "The keyboard layout.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "kb_variant",
                    "Keyboard Variant",
                    "The keyboard variant.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "kb_options",
                    "Keyboard Options",
                    "The keyboard options.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "kb_rules",
                    "Keyboard Rules",
                    "The keyboard rules.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "kb_file",
                    "Keyboard File",
                    "The keyboard file.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "numlock_by_default",
                    "Numlock by Default",
                    "Enables numlock by default.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "resolve_binds_by_sym",
                    "Resolve Binds by Symbol",
                    "Resolves binds by symbol.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "repeat_rate",
                    "Repeat Rate",
                    "The repeat rate.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "repeat_delay",
                    "Repeat Delay",
                    "The repeat delay.",
                );

                Self::add_section(&container, "Mouse Settings", "Configure mouse behavior.");
                Self::add_float_option(
                    &container,
                    &mut options,
                    "sensitivity",
                    "Sensitivity",
                    "The sensitivity.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "accel_profile",
                    "Acceleration Profile",
                    "The acceleration profile.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "force_no_accel",
                    "Force No Acceleration",
                    "Forces no acceleration.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "left_handed",
                    "Left Handed",
                    "Enables left handed mode.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "scroll_method",
                    "Scroll Method",
                    "The scroll method.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "scroll_button",
                    "Scroll Button",
                    "The scroll button.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "scroll_button_lock",
                    "Scroll Button Lock",
                    "Locks the scroll button.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "scroll_factor",
                    "Scroll Factor",
                    "The scroll factor.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "natural_scroll",
                    "Natural Scroll",
                    "Enables natural scroll.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "follow_mouse",
                    "Follow Mouse",
                    "Follows the mouse.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "mouse_refocus",
                    "Mouse Refocus",
                    "Refocuses on mouse.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "scroll_points",
                    "Scroll Points",
                    "The scroll points.",
                );

                Self::add_section(&container, "Focus Settings", "Configure focus behavior.");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "focus_on_close",
                    "Focus on Close",
                    "Focuses on close.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "float_switch_override_focus",
                    "Float Switch Override Focus",
                    "Overrides focus when switching to float.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "special_fallthrough",
                    "Special Fallthrough",
                    "Enables special fallthrough.",
                );

                Self::add_section(
                    &container,
                    "Touchpad Settings",
                    "Configure touchpad behavior.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:disable_while_typing",
                    "Disable While Typing",
                    "Disables the touchpad while typing.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:natural_scroll",
                    "Natural Scroll",
                    "Enables natural scroll.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "touchpad:scroll_factor",
                    "Scroll Factor",
                    "The scroll factor.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:middle_button_emulation",
                    "Middle Button Emulation",
                    "Emulates the middle button.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "touchpad:tap_button_map",
                    "Tap Button Map",
                    "The tap button map.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:clickfinger_behavior",
                    "Clickfinger Behavior",
                    "The clickfinger behavior.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:tap-to-click",
                    "Tap to Click",
                    "Enables tap to click.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:drag_lock",
                    "Drag Lock",
                    "Enables drag lock.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:tap-and-drag",
                    "Tap and Drag",
                    "Enables tap and drag.",
                );

                Self::add_section(
                    &container,
                    "Touchscreen Settings",
                    "Configure touchscreen behavior.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "touchdevice:transform",
                    "Transform",
                    "The transform.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "touchdevice:output",
                    "Output",
                    "The output.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "touchdevice:enabled",
                    "Enabled",
                    "Enables the touchdevice.",
                );

                Self::add_section(&container, "Tablet Settings", "Configure tablet behavior.");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "tablet:transform",
                    "Transform",
                    "The transform.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "tablet:output",
                    "Output",
                    "The output.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "tablet:region_position",
                    "Region Position",
                    "The region position.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "tablet:region_size",
                    "Region Size",
                    "The region size.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "tablet:relative_input",
                    "Relative Input",
                    "Enables relative input.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "tablet:left_handed",
                    "Left Handed",
                    "Enables left handed mode.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "tablet:active_area_size",
                    "Active Area Size",
                    "The active area size.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "tablet:active_area_position",
                    "Active Area Position",
                    "The active area position.",
                );

                Self::add_section(
                    &container,
                    "Miscellaneous Input Settings",
                    "Other input-related settings.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "off_window_axis_events",
                    "Off Window Axis Events",
                    "Enables off window axis events.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "emulate_discrete_scroll",
                    "Emulate Discrete Scroll",
                    "Emulates discrete scroll.",
                );
            }
            "gestures" => {
                Self::add_section(
                    &container,
                    "Gesture Settings",
                    "Configure gesture behavior.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe",
                    "Workspace Swipe",
                    "Enables workspace swipe.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_fingers",
                    "Workspace Swipe Fingers",
                    "The number of fingers for workspace swipe.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_min_fingers",
                    "Workspace Swipe Min Fingers",
                    "The minimum number of fingers for workspace swipe.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_distance",
                    "Workspace Swipe Distance",
                    "The distance for workspace swipe.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_touch",
                    "Workspace Swipe Touch",
                    "Enables workspace swipe touch.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_invert",
                    "Workspace Swipe Invert",
                    "Inverts workspace swipe.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_touch_invert",
                    "Workspace Swipe Touch Invert",
                    "Inverts workspace swipe touch.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_min_speed_to_force",
                    "Workspace Swipe Min Speed to Force",
                    "The minimum speed to force workspace swipe.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "workspace_swipe_cancel_ratio",
                    "Workspace Swipe Cancel Ratio",
                    "The cancel ratio for workspace swipe.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_create_new",
                    "Workspace Swipe Create New",
                    "Creates a new workspace on swipe.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_direction_lock",
                    "Workspace Swipe Direction Lock",
                    "Locks the direction for workspace swipe.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_direction_lock_threshold",
                    "Workspace Swipe Direction Lock Threshold",
                    "The threshold for workspace swipe direction lock.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_forever",
                    "Workspace Swipe Forever",
                    "Enables workspace swipe forever.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_use_r",
                    "Workspace Swipe Use R",
                    "Uses R for workspace swipe.",
                );
            }
            "Group" => {
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "auto_group",
                    "Auto Group",
                    "Enables auto group.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "insert_after_current",
                    "Insert After Current",
                    "Inserts after current.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "focus_removed_window",
                    "Focus Removed Window",
                    "Focuses the removed window.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "drag_into_group",
                    "Drag Into Group",
                    "Drags into group.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "merge_groups_on_drag",
                    "Merge Groups on Drag",
                    "Merges groups on drag.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "merge_floated_into_tiled_on_groupbar",
                    "Merge Floated Into Tiled on Groupbar",
                    "Merges floated into tiled on groupbar.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.border_active",
                    "Active Border Color",
                    "The color of the active border.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.border_inactive",
                    "Inactive Border Color",
                    "The color of the inactive border.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.border_locked_active",
                    "Locked Active Border Color",
                    "The color of the locked active border.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.border_locked_inactive",
                    "Locked Inactive Border Color",
                    "The color of the locked inactive border.",
                );

                Self::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:enabled",
                    "Enabled",
                    "Enables the groupbar.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:font_family",
                    "Font Family",
                    "The font family.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "groupbar:font_size",
                    "Font Size",
                    "The font size.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:gradients",
                    "Gradients",
                    "Enables gradients.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "groupbar:height",
                    "Height",
                    "The height.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:stacked",
                    "Stacked",
                    "Enables stacked.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "groupbar:priority",
                    "Priority",
                    "The priority.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:render_titles",
                    "Render Titles",
                    "Enables render titles.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:scrolling",
                    "Scrolling",
                    "Enables scrolling.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:text_color",
                    "Text Color",
                    "The text color.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:col.active",
                    "Active Color",
                    "The active color.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:col.inactive",
                    "Inactive Color",
                    "The inactive color.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:col.locked_active",
                    "Locked Active Color",
                    "The locked active color.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:col.locked_inactive",
                    "Locked Inactive Color",
                    "The locked inactive color.",
                );
            }
            "misc" => {
                Self::add_section(
                    &container,
                    "Miscellaneous Settings",
                    "Configure miscellaneous behavior.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_hyprland_logo",
                    "Disable Hyprland Logo",
                    "Disables the random Hyprland logo / anime girl background.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_splash_rendering",
                    "Disable Splash Rendering",
                    "Disables the Hyprland splash rendering.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "col.splash",
                    "Splash Color",
                    "Changes the color of the splash text.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "font_family",
                    "Font Family",
                    "Set the global default font to render the text.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "splash_font_family",
                    "Splash Font Family",
                    "Changes the font used to render the splash text.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "force_default_wallpaper",
                    "Force Default Wallpaper",
                    "Enforce any of the 3 default wallpapers.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "vfr",
                    "VFR",
                    "Controls the VFR status of Hyprland.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "vrr",
                    "VRR",
                    "Controls the VRR (Adaptive Sync) of your monitors.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "mouse_move_enables_dpms",
                    "Mouse Move Enables DPMS",
                    "If DPMS is set to off, wake up the monitors if the mouse moves.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "key_press_enables_dpms",
                    "Key Press Enables DPMS",
                    "If DPMS is set to off, wake up the monitors if a key is pressed.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "always_follow_on_dnd",
                    "Always Follow on DND",
                    "Will make mouse focus follow the mouse when drag and dropping.",
                );
                Self::add_bool_option(&container, &mut options, "layers_hog_keyboard_focus", "Layers Hog Keyboard Focus", "If true, will make keyboard-interactive layers keep their focus on mouse move.");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "animate_manual_resizes",
                    "Animate Manual Resizes",
                    "If true, will animate manual window resizes/moves.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "animate_mouse_windowdragging",
                    "Animate Mouse Window Dragging",
                    "If true, will animate windows being dragged by mouse.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_autoreload",
                    "Disable Autoreload",
                    "If true, the config will not reload automatically on save.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "enable_swallow",
                    "Enable Swallow",
                    "Enable window swallowing.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "swallow_regex",
                    "Swallow Regex",
                    "The class regex to be used for windows that should be swallowed.",
                );
                Self::add_string_option(
                    &container,
                    &mut options,
                    "swallow_exception_regex",
                    "Swallow Exception Regex",
                    "The title regex to be used for windows that should not be swallowed.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "focus_on_activate",
                    "Focus on Activate",
                    "Whether Hyprland should focus an app that requests to be focused.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "no_direct_scanout",
                    "No Direct Scanout",
                    "Disables direct scanout.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "mouse_move_focuses_monitor",
                    "Mouse Move Focuses Monitor",
                    "Whether mouse moving into a different monitor should focus it.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "suppress_portal_warnings",
                    "Suppress Portal Warnings",
                    "Disables warnings about incompatible portal implementations.",
                );
                Self::add_bool_option(&container, &mut options, "render_ahead_of_time", "Render Ahead of Time", "Starts rendering before your monitor displays a frame in order to lower latency.");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "render_ahead_safezone",
                    "Render Ahead Safezone",
                    "How many ms of safezone to add to rendering ahead of time.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "cursor_zoom_factor",
                    "Cursor Zoom Factor",
                    "The factor to zoom by around the cursor. Like a magnifying glass.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "cursor_zoom_rigid",
                    "Cursor Zoom Rigid",
                    "Whether the zoom should follow the cursor rigidly or loosely.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "allow_session_lock_restore",
                    "Allow Session Lock Restore",
                    "If true, will allow you to restart a lockscreen app in case it crashes.",
                );
                Self::add_color_option(
                    &container,
                    &mut options,
                    "background_color",
                    "Background Color",
                    "Change the background color.",
                );
            }
            "binds" => {
                Self::add_section(
                    &container,
                    "Bind Settings",
                    "Configure keybinding behavior.",
                );
                Self::add_bool_option(&container, &mut options, "pass_mouse_when_bound", "Pass Mouse When Bound", "If disabled, will not pass the mouse events to apps / dragging windows around if a keybind has been triggered.");
                Self::add_int_option(&container, &mut options, "scroll_event_delay", "Scroll Event Delay", "In ms, how many ms to wait after a scroll event to allow passing another one for the binds.");
                Self::add_bool_option(&container, &mut options, "workspace_back_and_forth", "Workspace Back and Forth", "If enabled, an attempt to switch to the currently focused workspace will instead switch to the previous workspace.");
                Self::add_bool_option(&container, &mut options, "allow_workspace_cycles", "Allow Workspace Cycles", "If enabled, workspaces don't forget their previous workspace, so cycles can be created.");
                Self::add_int_option(&container, &mut options, "workspace_center_on", "Workspace Center On", "Whether switching workspaces should center the cursor on the workspace (0) or on the last active window for that workspace (1).");
                Self::add_int_option(&container, &mut options, "focus_preferred_method", "Focus Preferred Method", "Sets the preferred focus finding method when using focuswindow/movewindow/etc with a direction.");
                Self::add_bool_option(&container, &mut options, "ignore_group_lock", "Ignore Group Lock", "If enabled, dispatchers like moveintogroup, moveoutofgroup and movewindoworgroup will ignore lock per group.");
                Self::add_bool_option(&container, &mut options, "movefocus_cycles_fullscreen", "Movefocus Cycles Fullscreen", "If enabled, when on a fullscreen window, movefocus will cycle fullscreen, if not, it will move the focus in a direction.");
                Self::add_bool_option(&container, &mut options, "disable_keybind_grabbing", "Disable Keybind Grabbing", "If enabled, apps that request keybinds to be disabled (e.g. VMs) will not be able to do so.");
                Self::add_bool_option(&container, &mut options, "window_direction_monitor_fallback", "Window Direction Monitor Fallback", "If enabled, moving a window or focus over the edge of a monitor with a direction will move it to the next monitor in that direction.");
            }
            "xwayland" => {
                Self::add_section(
                    &container,
                    "XWayland Settings",
                    "Configure XWayland behavior.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "enabled",
                    "Enabled",
                    "Allow running applications using X11.",
                );
                Self::add_bool_option(&container, &mut options, "use_nearest_neighbor", "Use Nearest Neighbor", "Uses the nearest neighbor filtering for xwayland apps, making them pixelated rather than blurry.");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "force_zero_scaling",
                    "Force Zero Scaling",
                    "Forces a scale of 1 on xwayland windows on scaled displays.",
                );
            }
            "opengl" => {
                Self::add_section(&container, "OpenGL Settings", "Configure OpenGL behavior.");
                Self::add_bool_option(&container, &mut options, "nvidia_anti_flicker", "Nvidia Anti Flicker", "Reduces flickering on nvidia at the cost of possible frame drops on lower-end GPUs.");
                Self::add_int_option(&container, &mut options, "force_introspection", "Force Introspection", "Forces introspection at all times. Introspection is aimed at reducing GPU usage in certain cases, but might cause graphical glitches on nvidia.");
            }
            "render" => {
                Self::add_section(&container, "Render Settings", "Configure render behavior.");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "explicit_sync",
                    "Explicit Sync",
                    "Whether to enable explicit sync support.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "explicit_sync_kms",
                    "Explicit Sync KMS",
                    "Whether to enable explicit sync support for the KMS layer.",
                );
                Self::add_bool_option(&container, &mut options, "direct_scanout", "Direct Scanout", "Enables direct scanout. Direct scanout attempts to reduce lag when there is only one fullscreen application on a screen.");
            }
            "cursor" => {
                Self::add_section(&container, "Cursor Settings", "Configure cursor behavior.");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "sync_gsettings_theme",
                    "Sync GSettings Theme",
                    "Sync xcursor theme with gsettings.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "no_hardware_cursors",
                    "No Hardware Cursors",
                    "Disables hardware cursors.",
                );
                Self::add_bool_option(&container, &mut options, "no_break_fs_vrr", "No Break FS VRR", "Disables scheduling new frames on cursor movement for fullscreen apps with VRR enabled to avoid framerate spikes.");
                Self::add_int_option(
                    &container,
                    &mut options,
                    "min_refresh_rate",
                    "Min Refresh Rate",
                    "Minimum refresh rate for cursor movement when no_break_fs_vrr is active.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "hotspot_padding",
                    "Hotspot Padding",
                    "The padding, in logical px, between screen edges and the cursor.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "inactive_timeout",
                    "Inactive Timeout",
                    "In seconds, after how many seconds of cursor's inactivity to hide it.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "no_warps",
                    "No Warps",
                    "If true, will not warp the cursor in many cases.",
                );
                Self::add_bool_option(&container, &mut options, "persistent_warps", "Persistent Warps", "When a window is refocused, the cursor returns to its last position relative to that window.");
                Self::add_bool_option(&container, &mut options, "warp_on_change_workspace", "Warp on Change Workspace", "If true, move the cursor to the last focused window after changing the workspace.");
                Self::add_string_option(
                    &container,
                    &mut options,
                    "default_monitor",
                    "Default Monitor",
                    "The name of a default monitor for the cursor to be set to on startup.",
                );
                Self::add_float_option(
                    &container,
                    &mut options,
                    "zoom_factor",
                    "Zoom Factor",
                    "The factor to zoom by around the cursor. Like a magnifying glass.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "zoom_rigid",
                    "Zoom Rigid",
                    "Whether the zoom should follow the cursor rigidly or loosely.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "enable_hyprcursor",
                    "Enable Hyprcursor",
                    "Whether to enable hyprcursor support.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "hide_on_key_press",
                    "Hide on Key Press",
                    "Hides the cursor when you press any key until the mouse is moved.",
                );
                Self::add_bool_option(&container, &mut options, "hide_on_touch", "Hide on Touch", "Hides the cursor when the last input was a touch input until a mouse input is done.");
                Self::add_bool_option(&container, &mut options, "allow_dumb_copy", "Allow Dumb Copy", "Makes HW cursors work on Nvidia, at the cost of a possible hitch whenever the image changes.");
            }
            "debug" => {
                Self::add_section(&container, "Debug Settings", "Configure debug behavior.");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "overlay",
                    "Overlay",
                    "Print the debug performance overlay.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "damage_blink",
                    "Damage Blink",
                    "Flash areas updated with damage tracking.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_logs",
                    "Disable Logs",
                    "Disable logging to a file.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_time",
                    "Disable Time",
                    "Disables time logging.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "damage_tracking",
                    "Damage Tracking",
                    "Redraw only the needed bits of the display.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "enable_stdout_logs",
                    "Enable Stdout Logs",
                    "Enables logging to stdout.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "manual_crash",
                    "Manual Crash",
                    "Set to 1 and then back to 0 to crash Hyprland.",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "suppress_errors",
                    "Suppress Errors",
                    "If true, do not display config file parsing errors.",
                );
                Self::add_int_option(&container, &mut options, "watchdog_timeout", "Watchdog Timeout", "Sets the timeout in seconds for watchdog to abort processing of a signal of the main thread.");
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "disable_scale_checks",
                    "Disable Scale Checks",
                    "Disables verification of the scale factors.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "error_limit",
                    "Error Limit",
                    "Limits the number of displayed config file parsing errors.",
                );
                Self::add_int_option(
                    &container,
                    &mut options,
                    "error_position",
                    "Error Position",
                    "Sets the position of the error bar. top - 0, bottom - 1",
                );
                Self::add_bool_option(
                    &container,
                    &mut options,
                    "colored_stdout_logs",
                    "Colored Stdout Logs",
                    "Enables colors in the stdout logs.",
                );
            }
            _ => {
                Self::add_section(
                    &container,
                    &format!("{} Settings", category),
                    &format!("Configure {} behavior.", category),
                );
            }
        }

        ConfigWidget { container, options }
    }

    fn add_section(container: &Box, title: &str, description: &str) {
        let section_box = Box::new(Orientation::Vertical, 5);
        section_box.set_margin_top(15);
        section_box.set_margin_bottom(10);

        let title_label = Label::new(Some(title));
        title_label.set_halign(gtk::Align::Start);
        title_label.set_markup(&format!("<b>{}</b>", title));
        section_box.append(&title_label);

        let desc_label = Label::new(Some(description));
        desc_label.set_halign(gtk::Align::Start);
        desc_label.set_opacity(0.7);
        section_box.append(&desc_label);

        let frame = Frame::new(None);
        frame.set_margin_top(10);
        section_box.append(&frame);

        container.append(&section_box);
    }

    fn add_int_option(
        container: &Box,
        options: &mut HashMap<String, Widget>,
        name: &str,
        label: &str,
        description: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.set_margin_start(10);
        hbox.set_margin_end(10);
        hbox.set_margin_top(5);
        hbox.set_margin_bottom(5);

        let vbox = Box::new(Orientation::Vertical, 2);
        let label_widget = Label::new(Some(label));
        label_widget.set_halign(gtk::Align::Start);
        vbox.append(&label_widget);

        let desc_label = Label::new(Some(description));
        desc_label.set_halign(gtk::Align::Start);
        desc_label.set_opacity(0.7);
        vbox.append(&desc_label);

        let spin_button = gtk::SpinButton::with_range(-1000.0, 1000.0, 1.0);
        spin_button.set_halign(gtk::Align::End);
        spin_button.set_hexpand(true);
        spin_button.set_width_request(100);

        hbox.append(&vbox);
        hbox.append(&spin_button);

        container.append(&hbox);

        options.insert(name.to_string(), spin_button.upcast());
    }

    fn add_bool_option(
        container: &Box,
        options: &mut HashMap<String, Widget>,
        name: &str,
        label: &str,
        description: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.set_margin_start(10);
        hbox.set_margin_end(10);
        hbox.set_margin_top(5);
        hbox.set_margin_bottom(5);

        let vbox = Box::new(Orientation::Vertical, 2);
        let label_widget = Label::new(Some(label));
        label_widget.set_halign(gtk::Align::Start);
        vbox.append(&label_widget);

        let desc_label = Label::new(Some(description));
        desc_label.set_halign(gtk::Align::Start);
        desc_label.set_opacity(0.7);
        vbox.append(&desc_label);

        let switch = Switch::new();
        switch.set_halign(gtk::Align::End);
        switch.set_valign(gtk::Align::Center);

        hbox.append(&vbox);
        hbox.append(&switch);

        container.append(&hbox);

        options.insert(name.to_string(), switch.upcast());
    }

    fn add_float_option(
        container: &Box,
        options: &mut HashMap<String, Widget>,
        name: &str,
        label: &str,
        description: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.set_margin_start(10);
        hbox.set_margin_end(10);
        hbox.set_margin_top(5);
        hbox.set_margin_bottom(5);

        let vbox = Box::new(Orientation::Vertical, 2);
        let label_widget = Label::new(Some(label));
        label_widget.set_halign(gtk::Align::Start);
        vbox.append(&label_widget);

        let desc_label = Label::new(Some(description));
        desc_label.set_halign(gtk::Align::Start);
        desc_label.set_opacity(0.7);
        vbox.append(&desc_label);

        let spin_button = gtk::SpinButton::with_range(-1000.0, 1000.0, 0.1);
        spin_button.set_digits(2);
        spin_button.set_halign(gtk::Align::End);
        spin_button.set_hexpand(true);
        spin_button.set_width_request(100);

        hbox.append(&vbox);
        hbox.append(&spin_button);

        container.append(&hbox);

        options.insert(name.to_string(), spin_button.upcast());
    }

    fn add_string_option(
        container: &Box,
        options: &mut HashMap<String, Widget>,
        name: &str,
        label: &str,
        description: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.set_margin_start(10);
        hbox.set_margin_end(10);
        hbox.set_margin_top(5);
        hbox.set_margin_bottom(5);

        let vbox = Box::new(Orientation::Vertical, 2);
        let label_widget = Label::new(Some(label));
        label_widget.set_halign(gtk::Align::Start);
        vbox.append(&label_widget);

        let desc_label = Label::new(Some(description));
        desc_label.set_halign(gtk::Align::Start);
        desc_label.set_opacity(0.7);
        vbox.append(&desc_label);

        let entry = Entry::new();
        entry.set_halign(gtk::Align::End);
        entry.set_hexpand(true);
        entry.set_width_request(100);

        let control_box = Box::new(Orientation::Horizontal, 5);
        control_box.set_halign(gtk::Align::End);
        control_box.set_hexpand(true);

        control_box.append(&entry);

        hbox.append(&vbox);
        hbox.append(&control_box);

        container.append(&hbox);

        options.insert(name.to_string(), entry.upcast());
    }

    fn add_color_option(
        container: &Box,
        options: &mut HashMap<String, Widget>,
        name: &str,
        label: &str,
        description: &str,
    ) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.set_margin_start(10);
        hbox.set_margin_end(10);
        hbox.set_margin_top(5);
        hbox.set_margin_bottom(5);

        let vbox = Box::new(Orientation::Vertical, 2);
        let label_widget = Label::new(Some(label));
        label_widget.set_halign(gtk::Align::Start);
        vbox.append(&label_widget);

        let desc_label = Label::new(Some(description));
        desc_label.set_halign(gtk::Align::Start);
        desc_label.set_opacity(0.7);
        vbox.append(&desc_label);

        let color_button = ColorButton::new();
        color_button.set_halign(gtk::Align::End);

        hbox.append(&vbox);
        hbox.append(&color_button);

        container.append(&hbox);

        options.insert(name.to_string(), color_button.upcast());
    }

    fn load_config(
        &self,
        config: &HyprlandConfig,
        category: &str,
        changed_options: Rc<RefCell<HashSet<(String, String)>>>,
    ) {
        for (name, widget) in &self.options {
            let value = self.extract_value(config, category, name);
            if let Some(spin_button) = widget.downcast_ref::<gtk::SpinButton>() {
                let int_value = value.parse::<f64>().unwrap_or(0.0);
                spin_button.set_value(int_value);
                let category = category.to_string();
                let name = name.to_string();
                let changed_options = changed_options.clone();
                let original_value = value.clone();
                spin_button.connect_value_changed(move |sb| {
                    let mut changes = changed_options.borrow_mut();
                    if sb.value().to_string() != original_value {
                        changes.insert((category.clone(), name.clone()));
                    } else {
                        changes.remove(&(category.clone(), name.clone()));
                    }
                });
            } else if let Some(entry) = widget.downcast_ref::<Entry>() {
                entry.set_text(&value);
                let category = category.to_string();
                let name = name.to_string();
                let changed_options = changed_options.clone();
                entry.connect_changed(move |entry| {
                    let mut changes = changed_options.borrow_mut();
                    if entry.text() != value {
                        changes.insert((category.clone(), name.clone()));
                    } else {
                        changes.remove(&(category.clone(), name.clone()));
                    }
                });
            } else if let Some(checkbox) = widget.downcast_ref::<CheckButton>() {
                checkbox.set_active(value == "true");
                let category = category.to_string();
                let name = name.to_string();
                let changed_options = changed_options.clone();
                checkbox.connect_toggled(move |cb| {
                    let mut changes = changed_options.borrow_mut();
                    if cb.is_active().to_string() != value {
                        changes.insert((category.clone(), name.clone()));
                    } else {
                        changes.remove(&(category.clone(), name.clone()));
                    }
                });
            } else if let Some(color_button) = widget.downcast_ref::<ColorButton>() {
                if let Some((red, green, blue, alpha)) = config.parse_color(&value) {
                    color_button.set_rgba(&gdk::RGBA::new(red, green, blue, alpha));
                }
                let category = category.to_string();
                let name = name.to_string();
                let changed_options = changed_options.clone();
                color_button.connect_color_set(move |cb| {
                    let mut changes = changed_options.borrow_mut();
                    let new_color = cb.rgba();
                    let new_value = format!(
                        "rgba({},{},{},{})",
                        new_color.red(),
                        new_color.green(),
                        new_color.blue(),
                        new_color.alpha()
                    );
                    if new_value != value {
                        changes.insert((category.clone(), name.clone()));
                    } else {
                        changes.remove(&(category.clone(), name.clone()));
                    }
                });
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

    fn apply_changes(
        &self,
        config: &mut HyprlandConfig,
        category: &str,
        changes: &HashSet<(String, String)>,
    ) {
        for (name, widget) in &self.options {
            if changes.contains(&(category.to_string(), name.to_string())) {
                let value = if let Some(spin_button) = widget.downcast_ref::<gtk::SpinButton>() {
                    spin_button.value().to_string()
                } else if let Some(entry) = widget.downcast_ref::<Entry>() {
                    entry.text().to_string()
                } else if let Some(checkbox) = widget.downcast_ref::<CheckButton>() {
                    checkbox.is_active().to_string()
                } else if let Some(color_button) = widget.downcast_ref::<ColorButton>() {
                    let rgba = color_button.rgba();
                    config.format_color(rgba.red(), rgba.green(), rgba.blue(), rgba.alpha())
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
    }
}
