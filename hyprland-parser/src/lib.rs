use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct HyprlandConfig {
    content: Vec<String>,
    sections: HashMap<String, usize>,
    added_entries: HashMap<String, HashSet<String>>,
}

impl HyprlandConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&mut self, config_str: &str) {
        let mut current_section = String::new();
        let mut section_start = 0;
        for (i, line) in config_str.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.ends_with('{') {
                current_section = trimmed.trim_end_matches('{').trim().to_string();
                section_start = i;
            } else if trimmed == "}" {
                if !current_section.is_empty() {
                    self.sections.insert(current_section.clone(), section_start);
                    current_section.clear();
                }
            }
            self.content.push(line.to_string());
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = self.content.join("\n");
        for (category, entries) in &self.added_entries {
            for entry in entries {
                if !self.entry_exists(category, entry) {
                    result.push_str(&format!("\n{}", entry));
                }
            }
        }
        result
    }

    fn entry_exists(&self, category: &str, entry: &str) -> bool {
        self.content.iter().any(|line| line.trim() == entry)
    }

    pub fn add_entry(&mut self, category: &str, entry: &str) {
        if !self.entry_exists(category, entry) {
            if let Some(&section_start) = self.sections.get(category) {
                let insert_position = self.content[section_start..]
                    .iter()
                    .position(|line| line.trim() == "}")
                    .map(|pos| section_start + pos)
                    .unwrap_or(self.content.len());
                self.content
                    .insert(insert_position, format!("    {}", entry));
            } else {
                self.added_entries
                    .entry(category.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(entry.to_string());
            }
        }
    }
}

pub fn parse_config(config_str: &str) -> HyprlandConfig {
    let mut config = HyprlandConfig::new();
    config.parse(config_str);
    config
}
