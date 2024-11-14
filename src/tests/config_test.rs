use crate::libs::config::read_config;

#[test]
fn test_read_config() {
    let config = read_config("static/template_config.toml").unwrap();
    println!("{:?}", config);
}
