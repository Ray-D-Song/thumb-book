use std::path::Path;
use std::env::current_dir;

const CONFIG_TEMPLATE: &str = include_str!("../../static/template_config.toml");

pub fn init() {
    // create config.toml
    let current_dir = current_dir().unwrap(); 
    let config_path = Path::new(&current_dir).join("config.toml");
    std::fs::write(config_path, CONFIG_TEMPLATE).unwrap();

    // create posts directory
    let posts_dir = Path::new(&current_dir).join("posts");
    std::fs::create_dir_all(posts_dir).unwrap();
}
