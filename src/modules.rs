use gtk::{prelude::*, Box, Orientation, ScrolledWindow, Widget};

use hyprparser::HyprlandConfig;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::gui::add_dropdown_option;
use crate::widgets::WidgetBuilder;

pub struct ConfigWidget {
    pub options: HashMap<String, Widget>,
    pub scrolled_window: ScrolledWindow,
}

impl ConfigWidget {
    pub fn new(category: &str) -> Self {
        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_vexpand(false);
        scrolled_window.set_propagate_natural_height(true);

        let container = Box::new(Orientation::Vertical, 0);
        container.set_margin_start(20);
        container.set_margin_end(20);
        container.set_margin_top(20);
        container.set_margin_bottom(20);

        scrolled_window.set_child(Some(&container));

        let mut options = HashMap::new();

        let first_section = Rc::new(RefCell::new(true));

        match category {
            "general" => {
                WidgetBuilder::add_section(
                    &container,
                    "General Settings",
                    "Configure general behavior.",
                    first_section.clone(),
                );

                WidgetBuilder::add_section(
                    &container,
                    "Layout",
                    "Choose the default layout.",
                    first_section.clone(),
                );
                add_dropdown_option(
                    &container,
                    &mut options,
                    "layout",
                    "Layout",
                    "which layout to use.",
                    &["dwindle", "master"],
                );
                WidgetBuilder::add_section(
                    &container,
                    "Gaps",
                    "Change gaps in & out, workspaces.",
                    first_section.clone(),
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "gaps_in",
                    "Gaps In",
                    "gaps between windows, also supports css style gaps (top, right, bottom, left -> 5,10,15,20)",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "gaps_out",
                    "Gaps Out",
                    "gaps between windows and monitor edges, also supports css style gaps (top, right, bottom, left -> 5,10,15,20)",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "gaps_workspaces",
                    "Gaps Workspaces",
                    "gaps between workspaces. Stacks with gaps_out.",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Borders",
                    "Size, resize, floating...",
                    first_section.clone(),
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "border_size",
                    "Border Size",
                    "size of the border around windows",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "no_border_on_floating",
                    "No Border on Floating",
                    "disable borders for floating windows",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "resize_on_border",
                    "Resize on Border",
                    "enables resizing windows by clicking and dragging on borders and gaps",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "extend_border_grab_area",
                    "Extend Border Grab Area",
                    "extends the area around the border where you can click and drag on, only used when general:resize_on_border is on.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "hover_icon_on_border",
                    "Hover Icon on Border",
                    "show a cursor icon when hovering over borders, only used when general:resize_on_border is on.",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Colors",
                    "Change borders colors.",
                    first_section.clone(),
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.inactive_border",
                    "Inactive Border Color",
                    "border color for inactive windows",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.active_border",
                    "Active Border Color",
                    "border color for the active window",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.nogroup_border",
                    "No Group Border Color",
                    "inactive border color for window that cannot be added to a group (see denywindowfromgroup dispatcher)",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.nogroup_border_active",
                    "No Group Active Border Color",
                    "active border color for window that cannot be added to a group",
                );
            }
            "decoration" => {
                WidgetBuilder::add_section(
                    &container,
                    "Window Decoration",
                    "Configure window appearance.",
                    first_section.clone(),
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "rounding",
                    "Rounding",
                    "rounded corners' radius (in layout px)",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "active_opacity",
                    "Active Opacity",
                    "opacity of active windows. [0.0 - 1.0]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "inactive_opacity",
                    "Inactive Opacity",
                    "opacity of inactive windows. [0.0 - 1.0]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "fullscreen_opacity",
                    "Fullscreen Opacity",
                    "opacity of fullscreen windows. [0.0 - 1.0]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "drop_shadow",
                    "Drop Shadow",
                    "enable drop shadows on windows",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "shadow_range",
                    "Shadow Range",
                    "Shadow range (\"size\") in layout px",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "shadow_render_power",
                    "Shadow Render Power",
                    "in what power to render the falloff (more power, the faster the falloff) [1 - 4]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "shadow_ignore_window",
                    "Shadow Ignore Window",
                    "if true, the shadow will not be rendered behind the window itself, only around it.",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.shadow",
                    "Shadow Color",
                    "shadow's color. Alpha dictates shadow's opacity.",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.shadow_inactive",
                    "Inactive Shadow Color",
                    "inactive shadow color. (if not set, will fall back to col.shadow)",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "shadow_offset",
                    "Shadow Offset",
                    "shadow's rendering offset. Format: \"x y\" (e.g. \"0 0\")",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "shadow_scale",
                    "Shadow Scale",
                    "shadow's scale. [0.0 - 1.0]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "dim_inactive",
                    "Dim Inactive",
                    "enables dimming of inactive windows",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "dim_strength",
                    "Dim Strength",
                    "how much inactive windows should be dimmed [0.0 - 1.0]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "dim_special",
                    "Dim Special",
                    "how much to dim the rest of the screen by when a special workspace is open. [0.0 - 1.0]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "dim_around",
                    "Dim Around",
                    "how much the dimaround window rule should dim by. [0.0 - 1.0]",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "screen_shader",
                    "Screen Shader",
                    "a path to a custom shader to be applied at the end of rendering. See examples/screenShader.frag for an example.",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Blur",
                    "Configure blur settings.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "blur:enabled",
                    "Blur Enabled",
                    "enable kawase window background blur",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "blur:size",
                    "Blur Size",
                    "blur size (distance)",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "blur:passes",
                    "Blur Passes",
                    "the amount of passes to perform",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "blur:ignore_opacity",
                    "Blur Ignore Opacity",
                    "make the blur layer ignore the opacity of the window",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "blur:new_optimizations",
                    "Blur New Optimizations",
                    "whether to enable further optimizations to the blur. Recommended to leave on, as it will massively improve performance.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "blur:xray",
                    "Blur X-Ray",
                    "if enabled, floating windows will ignore tiled windows in their blur. Only available if blur_new_optimizations is true. Will reduce overhead on floating blur significantly.",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "blur:noise",
                    "Blur Noise",
                    "how much noise to apply. [0.0 - 1.0]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "blur:contrast",
                    "Blur Contrast",
                    "contrast modulation for blur. [0.0 - 2.0]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "blur:brightness",
                    "Blur Brightness",
                    "brightness modulation for blur. [0.0 - 2.0]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "blur:vibrancy",
                    "Blur Vibrancy",
                    "Increase saturation of blurred colors. [0.0 - 1.0]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "blur:vibrancy_darkness",
                    "Blur Vibrancy Darkness",
                    "How strong the effect of vibrancy is on dark areas . [0.0 - 1.0]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "blur:special",
                    "Blur Special",
                    "whether to blur behind the special workspace (note: expensive)",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "blur:popups",
                    "Blur Popups",
                    "whether to blur popups (e.g. right-click menus)",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "blur:popups_ignorealpha",
                    "Blur Popups Ignore Alpha",
                    "works like ignorealpha in layer rules. If pixel opacity is below set value, will not blur. [0.0 - 1.0]",
                );
            }
            "animations" => {
                WidgetBuilder::add_section(
                    &container,
                    "Animation Settings",
                    "Configure animation behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "enabled",
                    "Enable Animations",
                    "Enables animations.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "first_launch_animation",
                    "First Launch Animation",
                    "Enables the first launch animation.",
                );
            }
            "input" => {
                WidgetBuilder::add_section(
                    &container,
                    "Input Settings",
                    "Configure input devices.",
                    first_section.clone(),
                );
                WidgetBuilder::add_section(
                    &container,
                    "Keyboard Settings",
                    "Configure keyboard behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "kb_model",
                    "Keyboard Model",
                    "Appropriate XKB keymap parameter.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "kb_layout",
                    "Keyboard Layout",
                    "Appropriate XKB keymap parameter",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "kb_variant",
                    "Keyboard Variant",
                    "Appropriate XKB keymap parameter",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "kb_options",
                    "Keyboard Options",
                    "Appropriate XKB keymap parameter",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "kb_rules",
                    "Keyboard Rules",
                    "Appropriate XKB keymap parameter",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "kb_file",
                    "Keyboard File",
                    "If you prefer, you can use a path to your custom .xkb file.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "numlock_by_default",
                    "Numlock by Default",
                    "Engage numlock by default.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "resolve_binds_by_sym",
                    "Resolve Binds by Symbol",
                    "Determines how keybinds act when multiple layouts are used. If false, keybinds will always act as if the first specified layout is active. If true, keybinds specified by symbols are activated when you type the respective symbol with the current layout.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "repeat_rate",
                    "Repeat Rate",
                    "The repeat rate for held-down keys, in repeats per second.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "repeat_delay",
                    "Repeat Delay",
                    "Delay before a held-down key is repeated, in milliseconds.",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Mouse Settings",
                    "Configure mouse behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "sensitivity",
                    "Sensitivity",
                    "Sets the mouse input sensitivity. Value is clamped to the range -1.0 to 1.0.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "accel_profile",
                    "Acceleration Profile",
                    "Sets the cursor acceleration profile. Can be one of adaptive, flat. Can also be custom, see below. Leave empty to use libinput's default mode for your input device.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "force_no_accel",
                    "Force No Acceleration",
                    "Force no cursor acceleration. This bypasses most of your pointer settings to get as raw of a signal as possible. Enabling this is not recommended due to potential cursor desynchronization.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "left_handed",
                    "Left Handed",
                    "Switches RMB and LMB",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "scroll_method",
                    "Scroll Method",
                    "Sets the scroll method. Can be one of 2fg (2 fingers), edge, on_button_down, no_scroll.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "scroll_button",
                    "Scroll Button",
                    "Sets the scroll button. Has to be an int, cannot be a string. Check wev if you have any doubts regarding the ID. 0 means default.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "scroll_button_lock",
                    "Scroll Button Lock",
                    "If the scroll button lock is enabled, the button does not need to be held down. Pressing and releasing the button toggles the button lock, which logically holds the button down or releases it. While the button is logically held down, motion events are converted to scroll events.",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "scroll_factor",
                    "Scroll Factor",
                    "Multiplier added to scroll movement for external mice. Note that there is a separate setting for touchpad scroll_factor.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "natural_scroll",
                    "Natural Scroll",
                    "Inverts scrolling direction. When enabled, scrolling moves content directly, rather than manipulating a scrollbar.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "follow_mouse",
                    "Follow Mouse",
                    "Specify if and how cursor movement should affect window focus. 0 - Cursor movement will not change focus, 1 - Cursor movement will always change focus to the window under the cursor, 2 - Cursor focus will be detached from keyboard focus, 3 - Cursor focus will be completely separate from keyboard focus. [0/1/2/3]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "mouse_refocus",
                    "Mouse Refocus",
                    "If disabled, mouse focus won't switch to the hovered window unless the mouse crosses a window boundary when follow_mouse=1.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "scroll_points",
                    "Scroll Points",
                    "Sets the scroll acceleration profile, when accel_profile is set to custom. Has to be in the form <step> <points>. Leave empty to have a flat scroll curve.",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Focus Settings",
                    "Configure focus behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "focus_on_close",
                    "Focus on Close",
                    "Controls the window focus behavior when a window is closed. 0 - focus will shift to the next window candidate, 1 - focus will shift to the window under the cursor. [0/1]",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "float_switch_override_focus",
                    "Float Switch Override Focus",
                    "If enabled, focus will change to the window under the cursor when changing from tiled-to-floating and vice versa. 0 - disabled, 1 - enabled, 2 - focus will also follow mouse on float-to-float switches. [0/1/2]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "special_fallthrough",
                    "Special Fallthrough",
                    "if enabled, having only floating windows in the special workspace will not block focusing windows in the regular workspace.",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Touchpad Settings",
                    "Configure touchpad behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:disable_while_typing",
                    "Disable While Typing",
                    "Disables the touchpad while typing.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:natural_scroll",
                    "Natural Scroll",
                    "Enables natural scroll.",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "touchpad:scroll_factor",
                    "Scroll Factor",
                    "The scroll factor.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:middle_button_emulation",
                    "Middle Button Emulation",
                    "Emulates the middle button.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "touchpad:tap_button_map",
                    "Tap Button Map",
                    "The tap button map.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:clickfinger_behavior",
                    "Clickfinger Behavior",
                    "The clickfinger behavior.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:tap-to-click",
                    "Tap to Click",
                    "Enables tap to click.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:drag_lock",
                    "Drag Lock",
                    "Enables drag lock.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "touchpad:tap-and-drag",
                    "Tap and Drag",
                    "Enables tap and drag.",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Touchscreen Settings",
                    "Configure touchscreen behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "touchdevice:transform",
                    "Transform",
                    "The transform.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "touchdevice:output",
                    "Output",
                    "The output.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "touchdevice:enabled",
                    "Enabled",
                    "Enables the touchdevice.",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Tablet Settings",
                    "Configure tablet behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "tablet:transform",
                    "Transform",
                    "The transform.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "tablet:output",
                    "Output",
                    "The output.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "tablet:region_position",
                    "Region Position",
                    "The region position.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "tablet:region_size",
                    "Region Size",
                    "The region size.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "tablet:relative_input",
                    "Relative Input",
                    "Enables relative input.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "tablet:left_handed",
                    "Left Handed",
                    "Enables left handed mode.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "tablet:active_area_size",
                    "Active Area Size",
                    "The active area size.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "tablet:active_area_position",
                    "Active Area Position",
                    "The active area position.",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Miscellaneous Input Settings",
                    "Other input-related settings.",
                    first_section.clone(),
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "off_window_axis_events",
                    "Off Window Axis Events",
                    "Handles axis events around a focused window. 0 - ignores axis events, 1 - sends out-of-bound coordinates, 2 - fakes pointer coordinates to the closest point inside the window, 3 - warps the cursor to the closest point inside the window [0/1/2/3]",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "emulate_discrete_scroll",
                    "Emulate Discrete Scroll",
                    "Emulates discrete scrolling from high resolution scrolling events. 0 - disables it, 1 - enables handling of non-standard events only, 2 - force enables all scroll wheel events to be handled [0/1/2]",
                );
            }
            "gestures" => {
                WidgetBuilder::add_section(
                    &container,
                    "Gesture Settings",
                    "Configure gesture behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe",
                    "Workspace Swipe",
                    "enable workspace swipe gesture on touchpad",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_fingers",
                    "Workspace Swipe Fingers",
                    "how many fingers for the touchpad gesture",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_min_fingers",
                    "Workspace Swipe Min Fingers",
                    "if enabled, workspace_swipe_fingers is considered the minimum number of fingers to swipe",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_distance",
                    "Workspace Swipe Distance",
                    "in px, the distance of the touchpad gesture",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_touch",
                    "Workspace Swipe Touch",
                    "enable workspace swiping from the edge of a touchscreen",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_invert",
                    "Workspace Swipe Invert",
                    "invert the direction (touchpad only)",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_touch_invert",
                    "Workspace Swipe Touch Invert",
                    "invert the direction (touchscreen only)",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_min_speed_to_force",
                    "Workspace Swipe Min Speed to Force",
                    "minimum speed in px per timepoint to force the change ignoring cancel_ratio. Setting to 0 will disable this mechanic.",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "workspace_swipe_cancel_ratio",
                    "Workspace Swipe Cancel Ratio",
                    "how much the swipe has to proceed in order to commence it. (0.7 -> if > 0.7 * distance, switch, if less, revert) [0.0 - 1.0]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_create_new",
                    "Workspace Swipe Create New",
                    "whether a swipe right on the last workspace should create a new one.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_direction_lock",
                    "Workspace Swipe Direction Lock",
                    "if enabled, switching direction will be locked when you swipe past the direction_lock_threshold (touchpad only).",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "workspace_swipe_direction_lock_threshold",
                    "Workspace Swipe Direction Lock Threshold",
                    "in px, the distance to swipe before direction lock activates (touchpad only).",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_forever",
                    "Workspace Swipe Forever",
                    "if enabled, swiping will not clamp at the neighboring workspaces but continue to the further ones.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_swipe_use_r",
                    "Workspace Swipe Use R",
                    "if enabled, swiping will use the r prefix instead of the m prefix for finding workspaces.",
                );
            }

            "group" => {
                WidgetBuilder::add_section(
                    &container,
                    "Group Settings",
                    "Configure group behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "auto_group",
                    "Auto Group",
                    "whether new windows will be automatically grouped into the focused unlocked group",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "insert_after_current",
                    "Insert After Current",
                    "whether new windows in a group spawn after current or at group tail",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "focus_removed_window",
                    "Focus Removed Window",
                    "whether Hyprland should focus on the window that has just been moved out of the group",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "drag_into_group",
                    "Drag Into Group",
                    "whether dragging a window into a unlocked group will merge them. 0 - disabled, 1 - enabled, 2 - only when dragging into the groupbar [0/1/2]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "merge_groups_on_drag",
                    "Merge Groups on Drag",
                    "whether window groups can be dragged into other groups",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "merge_floated_into_tiled_on_groupbar",
                    "Merge Floated Into Tiled on Groupbar",
                    "whether dragging a floating window into a tiled window groupbar will merge them",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.border_active",
                    "Active Border Color",
                    "active group border color",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.border_inactive",
                    "Inactive Border Color",
                    "inactive (out of focus) group border color",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.border_locked_active",
                    "Locked Active Border Color",
                    "active locked group border color",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.border_locked_inactive",
                    "Locked Inactive Border Color",
                    "inactive locked group border color",
                );
                WidgetBuilder::add_section(
                    &container,
                    "Groupbar Settings",
                    "Configure groupbar behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:enabled",
                    "Enabled",
                    "enables groupbars",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "groupbar:font_family",
                    "Font Family",
                    "font used to display groupbar titles, use misc:font_family if not specified",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "groupbar:font_size",
                    "Font Size",
                    "font size of groupbar title",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:gradients",
                    "Gradients",
                    "enables gradients",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "groupbar:height",
                    "Height",
                    "height of the groupbar",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:stacked",
                    "Stacked",
                    "render the groupbar as a vertical stack",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "groupbar:priority",
                    "Priority",
                    "sets the decoration priority for groupbars",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:render_titles",
                    "Render Titles",
                    "whether to render titles in the group bar decoration",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "groupbar:scrolling",
                    "Scrolling",
                    "whether scrolling in the groupbar changes group active window",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "groupbar:text_color",
                    "Text Color",
                    "controls the group bar text color",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "groupbar:col.active",
                    "Active Color",
                    "active group border color",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "groupbar:col.inactive",
                    "Inactive Color",
                    "inactive (out of focus) group border color",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "groupbar:col.locked_active",
                    "Locked Active Color",
                    "active locked group border color",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "groupbar:col.locked_inactive",
                    "Locked Inactive Color",
                    "inactive locked group border color",
                );
            }
            "misc" => {
                WidgetBuilder::add_section(
                    &container,
                    "Miscellaneous Settings",
                    "Configure miscellaneous behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "disable_hyprland_logo",
                    "Disable Hyprland Logo",
                    "disables the random Hyprland logo / anime girl background. :(",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "disable_splash_rendering",
                    "Disable Splash Rendering",
                    "disables the Hyprland splash rendering. (requires a monitor reload to take effect)",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "col.splash",
                    "Splash Color",
                    "Changes the color of the splash text (requires a monitor reload to take effect).",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "font_family",
                    "Font Family",
                    "Set the global default font to render the text including debug fps/notification, config error messages and etc., selected from system fonts.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "splash_font_family",
                    "Splash Font Family",
                    "Changes the font used to render the splash text, selected from system fonts (requires a monitor reload to take effect).",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "force_default_wallpaper",
                    "Force Default Wallpaper",
                    "Enforce any of the 3 default wallpapers. -1 - random, 0 or 1 - disables the anime background, 2 - enables anime background. [-1/0/1/2]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "vfr",
                    "VFR",
                    "controls the VFR status of Hyprland. Heavily recommended to leave enabled to conserve resources.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "vrr",
                    "VRR",
                    "Controls the VRR (Adaptive Sync) of your monitors. 0 - off, 1 - on, 2 - fullscreen only [0/1/2]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "mouse_move_enables_dpms",
                    "Mouse Move Enables DPMS",
                    "If DPMS is set to off, wake up the monitors if the mouse moves.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "key_press_enables_dpms",
                    "Key Press Enables DPMS",
                    "If DPMS is set to off, wake up the monitors if a key is pressed.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "always_follow_on_dnd",
                    "Always Follow on DnD",
                    "Will make mouse focus follow the mouse when drag and dropping. Recommended to leave it enabled, especially for people using focus follows mouse at 0.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "layers_hog_keyboard_focus",
                    "Layers Hog Keyboard Focus",
                    "If true, will make keyboard-interactive layers keep their focus on mouse move (e.g. wofi, bemenu)",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "animate_manual_resizes",
                    "Animate Manual Resizes",
                    "If true, will animate manual window resizes/moves",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "animate_mouse_windowdragging",
                    "Animate Mouse Window Dragging",
                    "If true, will animate windows being dragged by mouse, note that this can cause weird behavior on some curves",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "disable_autoreload",
                    "Disable Autoreload",
                    "If true, the config will not reload automatically on save, and instead needs to be reloaded with hyprctl reload. Might save on battery.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "enable_swallow",
                    "Enable Swallow",
                    "Enable window swallowing",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "swallow_regex",
                    "Swallow Regex",
                    "The class regex to be used for windows that should be swallowed (usually, a terminal). To know more about the list of regex which can be used use this cheatsheet.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "swallow_exception_regex",
                    "Swallow Exception Regex",
                    "The title regex to be used for windows that should not be swallowed by the windows specified in swallow_regex (e.g. wev). The regex is matched against the parent (e.g. Kitty) window's title on the assumption that it changes to whatever process it's running.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "focus_on_activate",
                    "Focus on Activate",
                    "Whether Hyprland should focus an app that requests to be focused (an activate request)",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "mouse_move_focuses_monitor",
                    "Mouse Move Focuses Monitor",
                    "Whether mouse moving into a different monitor should focus it",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "render_ahead_of_time",
                    "Render Ahead of Time",
                    "[Warning: buggy] starts rendering before your monitor displays a frame in order to lower latency"
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "render_ahead_safezone",
                    "Render Ahead Safezone",
                    "how many ms of safezone to add to rendering ahead of time. Recommended 1-2.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "allow_session_lock_restore",
                    "Allow Session Lock Restore",
                    "if true, will allow you to restart a lockscreen app in case it crashes (red screen of death)",
                );
                WidgetBuilder::add_color_option(
                    &container,
                    &mut options,
                    "background_color",
                    "Background Color",
                    "change the background color. (requires enabled disable_hyprland_logo)",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "close_special_on_empty",
                    "Close Special on Empty",
                    "close the special workspace if the last window is removed",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "new_window_takes_over_fullscreen",
                    "New Window Takes Over Fullscreen",
                    "If there is a fullscreen or maximized window, decide whether a new tiled window opened should replace it, stay behind or disable the fullscreen/maximized state. 0 - behind, 1 - takes over, 2 - unfullscreen/unmaxize [0/1/2]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "exit_window_retains_fullscreen",
                    "Exit Window Retains Fullscreen",
                    "if true, closing a fullscreen window makes the next focused window fullscreen",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "initial_workspace_tracking",
                    "Initial Workspace Tracking",
                    "If enabled, windows will open on the workspace they were invoked on. 0 - disabled, 1 - single-shot, 2 - persistent (all children too) [0/1/2]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "middle_click_paste",
                    "Middle Click Paste",
                    "whether to enable middle-click-paste (aka primary selection)",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "render_unfocused_fps",
                    "Render Unfocused FPS",
                    "the maximum limit for renderunfocused windows' fps in the background (see also Window-Rules - renderunfocused)",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "disable_xdg_env_checks",
                    "Disable XDG Environment Checks",
                    "disable the warning if XDG environment is externally managed",
                );
            }
            "binds" => {
                WidgetBuilder::add_section(
                    &container,
                    "Bind Settings",
                    "Configure keybinding behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "pass_mouse_when_bound",
                    "Pass Mouse When Bound",
                    "If disabled, will not pass the mouse events to apps / dragging windows around if a keybind has been triggered.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "scroll_event_delay",
                    "Scroll Event Delay",
                    "In ms, how many ms to wait after a scroll event to allow passing another one for the binds.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "workspace_back_and_forth",
                    "Workspace Back and Forth",
                    "If enabled, an attempt to switch to the currently focused workspace will instead switch to the previous workspace.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "allow_workspace_cycles",
                    "Allow Workspace Cycles",
                    "If enabled, workspaces don't forget their previous workspace, so cycles can be created.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "workspace_center_on",
                    "Workspace Center On",
                    "Whether switching workspaces should center the cursor on the workspace (0) or on the last active window for that workspace (1). [0/1]",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "focus_preferred_method",
                    "Focus Preferred Method",
                    "Sets the preferred focus finding method when using focuswindow/movewindow/etc with a direction. 0 - history (recent have priority), 1 - length (longer shared edges have priority) [0/1]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "ignore_group_lock",
                    "Ignore Group Lock",
                    "If enabled, dispatchers like moveintogroup, moveoutofgroup and movewindoworgroup will ignore lock per group.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "movefocus_cycles_fullscreen",
                    "Movefocus Cycles Fullscreen",
                    "If enabled, when on a fullscreen window, movefocus will cycle fullscreen, if not, it will move the focus in a direction.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "disable_keybind_grabbing",
                    "Disable Keybind Grabbing",
                    "If enabled, apps that request keybinds to be disabled (e.g. VMs) will not be able to do so.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "window_direction_monitor_fallback",
                    "Window Direction Monitor Fallback",
                    "If enabled, moving a window or focus over the edge of a monitor with a direction will move it to the next monitor in that direction.",
                );
            }
            "xwayland" => {
                WidgetBuilder::add_section(
                    &container,
                    "XWayland Settings",
                    "Configure XWayland behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "enabled",
                    "Enabled",
                    "Allow running applications using X11.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "use_nearest_neighbor",
                    "Use Nearest Neighbor",
                    "Uses the nearest neighbor filtering for xwayland apps, making them pixelated rather than blurry.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "force_zero_scaling",
                    "Force Zero Scaling",
                    "Forces a scale of 1 on xwayland windows on scaled displays.",
                );
            }
            "opengl" => {
                WidgetBuilder::add_section(
                    &container,
                    "OpenGL Settings",
                    "Configure OpenGL behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "nvidia_anti_flicker",
                    "Nvidia Anti Flicker",
                    "Reduces flickering on nvidia at the cost of possible frame drops on lower-end GPUs.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "force_introspection",
                    "Force Introspection",
                    "Forces introspection at all times. Introspection is aimed at reducing GPU usage in certain cases, but might cause graphical glitches on nvidia. 0 - nothing, 1 - force always on, 2 - force always on if nvidia [0/1/2]",
                );
            }
            "render" => {
                WidgetBuilder::add_section(
                    &container,
                    "Render Settings",
                    "Configure render behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "explicit_sync",
                    "Explicit Sync",
                    "Whether to enable explicit sync support. 0 - no, 1 - yes, 2 - auto based on the gpu driver [0/1/2]",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "explicit_sync_kms",
                    "Explicit Sync KMS",
                    "Whether to enable explicit sync support for the KMS layer. Requires explicit_sync to be enabled. 0 - no, 1 - yes, 2 - auto based on the gpu driver [0/1/2]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "direct_scanout",
                    "Direct Scanout",
                    "Enables direct scanout. Direct scanout attempts to reduce lag when there is only one fullscreen application on a screen.",
                );
            }
            "cursor" => {
                WidgetBuilder::add_section(
                    &container,
                    "Cursor Settings",
                    "Configure cursor behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "sync_gsettings_theme",
                    "Sync GSettings Theme",
                    "Sync xcursor theme with gsettings.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "no_hardware_cursors",
                    "No Hardware Cursors",
                    "Disables hardware cursors.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "no_break_fs_vrr",
                    "No Break FS VRR",
                    "Disables scheduling new frames on cursor movement for fullscreen apps with VRR enabled to avoid framerate spikes.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "min_refresh_rate",
                    "Min Refresh Rate",
                    "Minimum refresh rate for cursor movement when no_break_fs_vrr is active.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "hotspot_padding",
                    "Hotspot Padding",
                    "The padding, in logical px, between screen edges and the cursor.",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "inactive_timeout",
                    "Inactive Timeout",
                    "In seconds, after how many seconds of cursor's inactivity to hide it.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "no_warps",
                    "No Warps",
                    "If true, will not warp the cursor in many cases.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "persistent_warps",
                    "Persistent Warps",
                    "When a window is refocused, the cursor returns to its last position relative to that window.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "warp_on_change_workspace",
                    "Warp on Change Workspace",
                    "If true, move the cursor to the last focused window after changing the workspace.",
                );
                WidgetBuilder::add_string_option(
                    &container,
                    &mut options,
                    "default_monitor",
                    "Default Monitor",
                    "The name of a default monitor for the cursor to be set to on startup.",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "zoom_factor",
                    "Zoom Factor",
                    "The factor to zoom by around the cursor. Like a magnifying glass.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "zoom_rigid",
                    "Zoom Rigid",
                    "Whether the zoom should follow the cursor rigidly or loosely.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "enable_hyprcursor",
                    "Enable Hyprcursor",
                    "Whether to enable hyprcursor support.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "hide_on_key_press",
                    "Hide on Key Press",
                    "Hides the cursor when you press any key until the mouse is moved.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "hide_on_touch",
                    "Hide on Touch",
                    "Hides the cursor when the last input was a touch input until a mouse input is done.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "allow_dumb_copy",
                    "Allow Dumb Copy",
                    "Makes HW cursors work on Nvidia, at the cost of a possible hitch whenever the image changes.",
                );
            }
            "debug" => {
                WidgetBuilder::add_section(
                    &container,
                    "Debug Settings",
                    "Configure debug behavior.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "overlay",
                    "Overlay",
                    "Print the debug performance overlay.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "damage_blink",
                    "Damage Blink",
                    "(epilepsy warning!) Flash areas updated with damage tracking.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "disable_logs",
                    "Disable Logs",
                    "Disable logging to a file.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "disable_time",
                    "Disable Time",
                    "Disables time logging.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "damage_tracking",
                    "Damage Tracking",
                    "Redraw only the needed bits of the display. Do not change. 0 - none, 1 - monitor, 2 - full (default) [0/1/2]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "enable_stdout_logs",
                    "Enable Stdout Logs",
                    "Enables logging to stdout.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "manual_crash",
                    "Manual Crash",
                    "Set to 1 and then back to 0 to crash Hyprland.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "suppress_errors",
                    "Suppress Errors",
                    "If true, do not display config file parsing errors.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "watchdog_timeout",
                    "Watchdog Timeout",
                    "Sets the timeout in seconds for watchdog to abort processing of a signal of the main thread. Set to 0 to disable.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "disable_scale_checks",
                    "Disable Scale Checks",
                    "Disables verification of the scale factors. Will result in pixel alignment and rounding errors.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "error_limit",
                    "Error Limit",
                    "Limits the number of displayed config file parsing errors.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "error_position",
                    "Error Position",
                    "Sets the position of the error bar. 0 - top, 1 - bottom [0/1]",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "colored_stdout_logs",
                    "Colored Stdout Logs",
                    "Enables colors in the stdout logs.",
                );
            }
            "layouts" => {
                WidgetBuilder::add_section(
                    &container,
                    "Layout Settings",
                    "Configure layout behavior.",
                    first_section.clone(),
                );

                WidgetBuilder::add_section(
                    &container,
                    "Dwindle Layout",
                    "Configure Dwindle layout settings.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "dwindle:pseudotile",
                    "Pseudotile",
                    "Enable pseudotiling. Pseudotiled windows retain their floating size when tiled.",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "dwindle:force_split",
                    "Force Split",
                    "0 -> split follows mouse, 1 -> always split to the left (new = left or top) 2 -> always split to the right (new = right or bottom)",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "dwindle:preserve_split",
                    "Preserve Split",
                    "If enabled, the split (side/top) will not change regardless of what happens to the container.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "dwindle:smart_split",
                    "Smart Split",
                    "If enabled, allows a more precise control over the window split direction based on the cursor's position.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "dwindle:smart_resizing",
                    "Smart Resizing",
                    "If enabled, resizing direction will be determined by the mouse's position on the window.",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "dwindle:permanent_direction_override",
                    "Permanent Direction Override",
                    "If enabled, makes the preselect direction persist until changed or disabled.",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "dwindle:special_scale_factor",
                    "Special Scale Factor",
                    "Specifies the scale factor of windows on the special workspace [0 - 1]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "dwindle:split_width_multiplier",
                    "Split Width Multiplier",
                    "Specifies the auto-split width multiplier",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "dwindle:use_active_for_splits",
                    "Use Active for Splits",
                    "Whether to prefer the active window or the mouse position for splits",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "dwindle:default_split_ratio",
                    "Default Split Ratio",
                    "The default split ratio on window open. 1 means even 50/50 split. [0.1 - 1.9]",
                );
                WidgetBuilder::add_int_option(
                    &container,
                    &mut options,
                    "dwindle:split_bias",
                    "Split Bias",
                    "Specifies which window will receive the larger half of a split. [0/1/2]",
                );

                WidgetBuilder::add_section(
                    &container,
                    "Master Layout",
                    "Configure Master layout settings.",
                    first_section.clone(),
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "master:allow_small_split",
                    "Allow Small Split",
                    "Enable adding additional master windows in a horizontal split style",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "master:special_scale_factor",
                    "Special Scale Factor",
                    "The scale of the special workspace windows. [0.0 - 1.0]",
                );
                WidgetBuilder::add_float_option(
                    &container,
                    &mut options,
                    "master:mfact",
                    "Master Factor",
                    "The size as a percentage of the master window. [0.0 - 1.0]",
                );
                add_dropdown_option(
                    &container,
                    &mut options,
                    "master:new_status",
                    "New Window Status",
                    "Determines how new windows are added to the layout.",
                    &["master", "slave", "inherit"],
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "master:new_on_top",
                    "New on Top",
                    "Whether a newly open window should be on the top of the stack",
                );
                add_dropdown_option(
                    &container,
                    &mut options,
                    "master:new_on_active",
                    "New on Active",
                    "Place new window relative to the focused window",
                    &["before", "after", "none"],
                );
                add_dropdown_option(
                    &container,
                    &mut options,
                    "master:orientation",
                    "Orientation",
                    "Default placement of the master area",
                    &["left", "right", "top", "bottom", "center"],
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "master:inherit_fullscreen",
                    "Inherit Fullscreen",
                    "Inherit fullscreen status when cycling/swapping to another window",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "master:always_center_master",
                    "Always Center Master",
                    "Keep the master window centered when using center orientation",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "master:smart_resizing",
                    "Smart Resizing",
                    "If enabled, resizing direction will be determined by the mouse's position on the window",
                );
                WidgetBuilder::add_bool_option(
                    &container,
                    &mut options,
                    "master:drop_at_cursor",
                    "Drop at Cursor",
                    "When enabled, dragging and dropping windows will put them at the cursor position",
                );
            }
            _ => {
                WidgetBuilder::add_section(
                    &container,
                    &format!("{} Settings", category),
                    &format!("Configure {} behavior.", category),
                    first_section.clone(),
                );
            }
        }

        ConfigWidget {
            options,
            scrolled_window,
        }
    }

    pub fn load_config(
        &self,
        config: &HyprlandConfig,
        category: &str,
        changed_options: Rc<RefCell<HashMap<(String, String), String>>>,
    ) {
        let mut builder = WidgetBuilder::new();
        builder.options = self.options.clone();
        builder.load_config(config, category, changed_options);
    }
}
