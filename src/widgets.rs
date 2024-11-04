use gtk::{
    gdk, prelude::*, Box, Button, ColorButton, Entry, Frame, Image, Label, Orientation, Popover,
    SpinButton, Switch, Widget,
};
use hyprparser::HyprlandConfig;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::gui::get_option_limits;

pub struct WidgetBuilder {
    pub options: HashMap<String, Widget>,
}

impl WidgetBuilder {
    pub fn new() -> Self {
        Self {
            options: HashMap::new(),
        }
    }

    pub fn add_section(
        container: &Box,
        title: &str,
        description: &str,
        first_section: Rc<RefCell<bool>>,
    ) {
        let section_box = Box::new(Orientation::Vertical, 5);
        section_box.set_margin_top(15);
        section_box.set_margin_bottom(10);

        let title_label = Label::new(Some(title));
        let desc_label = Label::new(Some(description));

        if *first_section.borrow() {
            title_label.set_halign(gtk::Align::Center);
            desc_label.set_halign(gtk::Align::Center);
            title_label.set_hexpand(true);
            desc_label.set_hexpand(true);
            *first_section.borrow_mut() = false;
        } else {
            title_label.set_halign(gtk::Align::Start);
            desc_label.set_halign(gtk::Align::Start);
        }

        title_label.set_markup(&format!("<b>{}</b>", title));
        section_box.append(&title_label);

        desc_label.set_opacity(0.7);
        section_box.append(&desc_label);

        let frame = Frame::new(None);
        frame.set_margin_top(10);
        section_box.append(&frame);

        container.append(&section_box);
    }

    pub fn add_int_option(
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

        let (min, max, step) = get_option_limits(name, description);
        let spin_button = SpinButton::with_range(min, max, step);
        spin_button.set_digits(0);
        spin_button.set_halign(gtk::Align::End);
        spin_button.set_width_request(100);

        hbox.append(&label_box);
        hbox.append(&spin_button);

        container.append(&hbox);

        options.insert(name.to_string(), spin_button.upcast());
    }

    pub fn add_bool_option(
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

        let switch = Switch::new();
        switch.set_halign(gtk::Align::End);
        switch.set_valign(gtk::Align::Center);

        hbox.append(&label_box);
        hbox.append(&switch);

        container.append(&hbox);

        options.insert(name.to_string(), switch.upcast());
    }

    pub fn add_float_option(
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

        let (min, max, step) = get_option_limits(name, description);
        let spin_button = SpinButton::with_range(min, max, step);
        spin_button.set_digits(2);
        spin_button.set_halign(gtk::Align::End);
        spin_button.set_width_request(100);

        hbox.append(&label_box);
        hbox.append(&spin_button);

        container.append(&hbox);

        options.insert(name.to_string(), spin_button.upcast());
    }

    pub fn add_string_option(
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

        let entry = Entry::new();
        entry.set_halign(gtk::Align::End);
        entry.set_width_request(100);

        hbox.append(&label_box);
        hbox.append(&entry);

        container.append(&hbox);

        options.insert(name.to_string(), entry.upcast());
    }

    pub fn add_color_option(
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

        let color_button = ColorButton::new();
        color_button.set_halign(gtk::Align::End);

        hbox.append(&label_box);
        hbox.append(&color_button);

        container.append(&hbox);

        options.insert(name.to_string(), color_button.upcast());
    }

    pub fn load_config(
        &self,
        config: &HyprlandConfig,
        category: &str,
        changed_options: Rc<RefCell<HashMap<(String, String), String>>>,
    ) {
        for (name, widget) in &self.options {
            let value = self.extract_value(config, category, name);
            if let Some(spin_button) = widget.downcast_ref::<gtk::SpinButton>() {
                let float_value = value.parse::<f64>().unwrap_or(0.0);
                spin_button.set_value(float_value);
                let category = category.to_string();
                let name = name.to_string();
                let changed_options = changed_options.clone();
                spin_button.connect_value_changed(move |sb| {
                    let mut changes = changed_options.borrow_mut();
                    let new_value = sb.value().to_string();
                    changes.insert((category.clone(), name.clone()), new_value);
                });
            } else if let Some(entry) = widget.downcast_ref::<Entry>() {
                entry.set_text(&value);
                let category = category.to_string();
                let name = name.to_string();
                let changed_options = changed_options.clone();
                entry.connect_changed(move |entry| {
                    let mut changes = changed_options.borrow_mut();
                    let new_value = entry.text().to_string();
                    changes.insert((category.clone(), name.clone()), new_value);
                });
            } else if let Some(switch) = widget.downcast_ref::<Switch>() {
                switch.set_active(value == "true");
                let category = category.to_string();
                let name = name.to_string();
                let changed_options = changed_options.clone();
                switch.connect_active_notify(move |sw| {
                    let mut changes = changed_options.borrow_mut();
                    let new_value = sw.is_active().to_string();
                    changes.insert((category.clone(), name.clone()), new_value);
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
                        "rgba({:02X}{:02X}{:02X}{:02X})",
                        (new_color.red() * 255.0) as u8,
                        (new_color.green() * 255.0) as u8,
                        (new_color.blue() * 255.0) as u8,
                        (new_color.alpha() * 255.0) as u8
                    );
                    changes.insert((category.clone(), name.clone()), new_value);
                });
            } else if let Some(dropdown) = widget.downcast_ref::<gtk::DropDown>() {
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
                let category = category.to_string();
                let name = name.to_string();
                let changed_options = changed_options.clone();
                dropdown.connect_selected_notify(move |dd| {
                    let mut changes = changed_options.borrow_mut();
                    if let Some(selected) = dd.selected_item() {
                        if let Some(string_object) = selected.downcast_ref::<gtk::StringObject>() {
                            let new_value = string_object.string().to_string();
                            changes.insert((category.clone(), name.clone()), new_value);
                        }
                    }
                });
            }
        }
    }

    pub fn extract_value(&self, config: &HyprlandConfig, category: &str, name: &str) -> String {
        let mut value = String::new();
        let parts: Vec<&str> = name.split(':').collect();

        if parts.len() > 1 {
            if let Some(&(parent_start, parent_end)) = config.sections.get(category) {
                if parent_start < config.content.len() && parent_end < config.content.len() {
                    let subsection = format!("{} {{", parts[0]);
                    let mut in_subsection = false;

                    for line in &config.content[parent_start..=parent_end] {
                        let trimmed = line.trim();

                        if trimmed == subsection {
                            in_subsection = true;
                            continue;
                        }

                        if in_subsection {
                            if trimmed == "}" {
                                break;
                            }

                            if trimmed.starts_with(parts[1])
                                && trimmed[parts[1].len()..].trim_start().starts_with('=')
                            {
                                if let Some(val) = trimmed.split('=').nth(1) {
                                    value = val.trim().to_string();
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        } else if let Some(&(start, end)) = config.sections.get(category) {
            if start < config.content.len() && end < config.content.len() {
                for line in &config.content[start..=end] {
                    let trimmed = line.trim();
                    if trimmed.starts_with(name)
                        && trimmed[name.len()..].trim_start().starts_with('=')
                    {
                        if let Some(val) = line.split('=').nth(1) {
                            value = val.trim().to_string();
                            break;
                        }
                    }
                }
            }
        }

        if value.is_empty() {
            for idx in 0..config.sourced_content.len() {
                let section_key = format!("{}_{}", category, idx);
                if let Some(&(start, end)) = config.sourced_sections.get(&section_key) {
                    let sourced = &config.sourced_content[idx];
                    if start < sourced.len() && end < sourced.len() {
                        if parts.len() > 1 {
                            let subsection = format!("{} {{", parts[0]);
                            let mut in_subsection = false;

                            for line in &sourced[start..=end] {
                                let trimmed = line.trim();

                                if trimmed == subsection {
                                    in_subsection = true;
                                    continue;
                                }

                                if in_subsection {
                                    if trimmed == "}" {
                                        break;
                                    }

                                    if trimmed.starts_with(parts[1])
                                        && trimmed[parts[1].len()..].trim_start().starts_with('=')
                                    {
                                        if let Some(val) = trimmed.split('=').nth(1) {
                                            value = val.trim().to_string();
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            for line in &sourced[start..=end] {
                                let trimmed = line.trim();
                                if trimmed.starts_with(name)
                                    && trimmed[name.len()..].trim_start().starts_with('=')
                                {
                                    if let Some(val) = line.split('=').nth(1) {
                                        value = val.trim().to_string();
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                if !value.is_empty() {
                    break;
                }
            }
        }

        value
    }
}
