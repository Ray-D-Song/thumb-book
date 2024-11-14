use std::path::Path;
use std::env::current_dir;

pub fn check_dir() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = current_dir().unwrap();
    // check if the config file exists, if not, panic and remind the user to run the init command
    let config_path = Path::new(&current_dir).join("config.toml");
    if !std::fs::metadata(&config_path).is_ok() {
        panic!("Config file not found, please run the init command to create a new config file");
    }
    let posts_dir = Path::new(&current_dir).join("posts");
    let posts_output_dir = Path::new(&current_dir).join("posts_output");
    // check above paths exist
    if !std::fs::metadata(&posts_dir).is_ok() {
        panic!("Posts directory not found, please run the init command to create a new posts directory");
    }

    // check if posts_output_dir exists, if not, create it
    if !std::fs::metadata(&posts_output_dir).is_ok() {
        std::fs::create_dir_all(&posts_output_dir).unwrap();
    }
    Ok(())
}