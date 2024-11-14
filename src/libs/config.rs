use std::error::Error;
use std::collections::HashMap;
use toml;

pub fn read_config(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let config = toml::from_str(&std::fs::read_to_string(path)?)?;
    Ok(config)
}
