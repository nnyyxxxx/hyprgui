use hyprland_parser::parse_config;
use std::fs;
use std::path::PathBuf;

fn main() {
    let home = std::env::var("HOME").unwrap();
    let config_path = PathBuf::from(home).join(".config/hypr/hyprland.conf");

    let config_str = fs::read_to_string(&config_path).unwrap();

    let mut parsed_config = parse_config(&config_str);

    parsed_config.add_entry("general", "new_option = value");
    parsed_config.add_entry("exec", "some_command --with-args");
    parsed_config.add_entry("bind", "$mod, T, exec, kitty");

    let updated_config_str = parsed_config.to_string();

    fs::write(&config_path, updated_config_str).unwrap();

    println!("Updated hyprland.conf with new configurations.");
}
