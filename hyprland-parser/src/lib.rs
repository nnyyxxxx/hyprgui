use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct HyprlandConfig {
    content: Vec<String>,
    sections: HashMap<String, (usize, usize)>,
}

impl HyprlandConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&mut self, config_str: &str) {
        let mut section_stack = Vec::new();
        for (i, line) in config_str.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.ends_with('{') {
                let section_name = trimmed.trim_end_matches('{').trim().to_string();
                section_stack.push((section_name, i));
            } else if trimmed == "}" && !section_stack.is_empty() {
                let (name, start) = section_stack.pop().unwrap();
                let full_name = section_stack
                    .iter()
                    .map(|(n, _)| n.as_str())
                    .chain(std::iter::once(name.as_str()))
                    .collect::<Vec<_>>()
                    .join(".");
                self.sections.insert(full_name, (start, i));
            }
            self.content.push(line.to_string());
        }
    }

    pub fn to_string(&self) -> String {
        self.content.join("\n")
    }

    pub fn add_entry(&mut self, category: &str, entry: &str) {
        let parts: Vec<&str> = category.split('.').collect();
        let mut current_section = String::new();
        let mut depth = 0;
        let mut insert_pos = self.content.len();

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                current_section.push('.');
            }
            current_section.push_str(part);

            if let Some(&(start, end)) = self.sections.get(&current_section) {
                insert_pos = end;
                if i == parts.len() - 1 {
                    let key = entry.split('=').next().unwrap().trim();
                    let existing_line = self.content[start..=end]
                        .iter()
                        .position(|line| line.trim().starts_with(key))
                        .map(|pos| start + pos);

                    if let Some(line_num) = existing_line {
                        self.content[line_num] = format!("{}{}", "    ".repeat(depth + 1), entry);
                    } else {
                        self.content
                            .insert(end, format!("{}{}", "    ".repeat(depth + 1), entry));
                        self.update_sections(end, 1);
                    }
                    return;
                }
            } else {
                let new_section = format!("{}{} {{", "    ".repeat(depth), part);
                self.content.insert(insert_pos, new_section);
                insert_pos += 1;
                if i == parts.len() - 1 {
                    self.content
                        .insert(insert_pos, format!("{}{}", "    ".repeat(depth + 1), entry));
                    insert_pos += 1;
                }
                self.content
                    .insert(insert_pos, format!("{}}}", "    ".repeat(depth)));

                self.update_sections(insert_pos - 2, 3);
                self.sections
                    .insert(current_section.clone(), (insert_pos - 2, insert_pos));
                insert_pos += 1;
            }

            depth += 1;
        }
    }

    fn update_sections(&mut self, pos: usize, offset: usize) {
        for (start, end) in self.sections.values_mut() {
            if *start >= pos {
                *start += offset;
                *end += offset;
            } else if *end >= pos {
                *end += offset;
            }
        }
    }
}

pub fn parse_config(config_str: &str) -> HyprlandConfig {
    let mut config = HyprlandConfig::new();
    config.parse(config_str);
    config
}
