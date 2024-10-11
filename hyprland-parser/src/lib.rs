use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct HyprlandConfig {
    general: HashMap<String, String>,
    decoration: HashMap<String, String>,
    animations: HashMap<String, String>,
    input: HashMap<String, String>,
    gestures: HashMap<String, String>,
    group: HashMap<String, String>,
    misc: HashMap<String, String>,
    binds: HashMap<String, String>,
    xwayland: HashMap<String, String>,
    opengl: HashMap<String, String>,
    render: HashMap<String, String>,
    cursor: HashMap<String, String>,
    debug: HashMap<String, String>,
    blur: HashMap<String, String>,
    touchpad: HashMap<String, String>,
    touchdevice: HashMap<String, String>,
    tablet: HashMap<String, String>,
    groupbar: HashMap<String, String>,
    exec: Vec<String>,
    exec_once: Vec<String>,
    monitor: Vec<String>,
    windowrule: Vec<String>,
    windowrulev2: Vec<String>,
    bind: Vec<String>,
    bindm: Vec<String>,
}

impl HyprlandConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&mut self, config_str: &str) {
        let mut section_stack = Vec::new();

        for line in config_str.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.ends_with('{') {
                let section = trimmed.trim_end_matches('{').trim().to_string();
                section_stack.push(section);
                continue;
            }

            if trimmed == "}" {
                section_stack.pop();
                continue;
            }

            if trimmed.starts_with("exec = ") {
                self.exec.push(trimmed[6..].to_string());
            } else if trimmed.starts_with("exec-once = ") {
                self.exec_once.push(trimmed[11..].to_string());
            } else if trimmed.starts_with("monitor = ") {
                self.monitor.push(trimmed[9..].to_string());
            } else if trimmed.starts_with("windowrule = ") {
                self.windowrule.push(trimmed[12..].to_string());
            } else if trimmed.starts_with("windowrulev2 = ") {
                self.windowrulev2.push(trimmed[14..].to_string());
            } else if trimmed.starts_with("bind = ") {
                self.bind.push(trimmed[6..].to_string());
            } else if trimmed.starts_with("bindm = ") {
                self.bindm.push(trimmed[7..].to_string());
            } else {
                let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    let current_section = section_stack.join(":");
                    match current_section.as_str() {
                        "general" => self.general.insert(key.to_string(), value.to_string()),
                        "decoration" => self.decoration.insert(key.to_string(), value.to_string()),
                        "animations" => self.animations.insert(key.to_string(), value.to_string()),
                        "input" => self.input.insert(key.to_string(), value.to_string()),
                        "gestures" => self.gestures.insert(key.to_string(), value.to_string()),
                        "group" => self.group.insert(key.to_string(), value.to_string()),
                        "misc" => self.misc.insert(key.to_string(), value.to_string()),
                        "binds" => self.binds.insert(key.to_string(), value.to_string()),
                        "xwayland" => self.xwayland.insert(key.to_string(), value.to_string()),
                        "opengl" => self.opengl.insert(key.to_string(), value.to_string()),
                        "render" => self.render.insert(key.to_string(), value.to_string()),
                        "cursor" => self.cursor.insert(key.to_string(), value.to_string()),
                        "debug" => self.debug.insert(key.to_string(), value.to_string()),
                        "decoration:blur" => self.blur.insert(key.to_string(), value.to_string()),
                        "input:touchpad" => {
                            self.touchpad.insert(key.to_string(), value.to_string())
                        }
                        "input:touchdevice" => {
                            self.touchdevice.insert(key.to_string(), value.to_string())
                        }
                        "input:tablet" => self.tablet.insert(key.to_string(), value.to_string()),
                        "group:groupbar" => {
                            self.groupbar.insert(key.to_string(), value.to_string())
                        }
                        _ => None,
                    };
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut config = String::new();

        for (key, value) in &self.general {
            config.push_str(&format!("{} = {}\n", key, value));
        }

        let sections = [
            ("decoration", &self.decoration),
            ("animations", &self.animations),
            ("input", &self.input),
            ("gestures", &self.gestures),
            ("group", &self.group),
            ("misc", &self.misc),
            ("binds", &self.binds),
            ("xwayland", &self.xwayland),
            ("opengl", &self.opengl),
            ("render", &self.render),
            ("cursor", &self.cursor),
            ("debug", &self.debug),
        ];

        for (section, map) in sections {
            if !map.is_empty() {
                config.push_str(&format!("\n{} {{\n", section));
                for (key, value) in map {
                    config.push_str(&format!("    {} = {}\n", key, value));
                }
                config.push_str("}\n");
            }
        }

        if !self.blur.is_empty() {
            config.push_str("\ndecoration:blur {\n");
            for (key, value) in &self.blur {
                config.push_str(&format!("    {} = {}\n", key, value));
            }
            config.push_str("}\n");
        }

        let lists = [
            ("exec", &self.exec),
            ("exec-once", &self.exec_once),
            ("monitor", &self.monitor),
            ("windowrule", &self.windowrule),
            ("windowrulev2", &self.windowrulev2),
            ("bind", &self.bind),
            ("bindm", &self.bindm),
        ];

        for (prefix, list) in lists {
            for item in list {
                config.push_str(&format!("{} = {}\n", prefix, item));
            }
        }

        config
    }

    pub fn insert_general(&mut self, key: String, value: String) {
        self.general.insert(key, value);
    }

    pub fn add_exec(&mut self, command: String) {
        self.exec.push(command);
    }

    pub fn add_bind(&mut self, binding: String) {
        self.bind.push(binding);
    }
}

pub fn parse_config(config_str: &str) -> HyprlandConfig {
    let mut config = HyprlandConfig::new();
    config.parse(config_str);
    config
}
