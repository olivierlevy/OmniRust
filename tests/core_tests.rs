use omnirust::core::config::Config;

#[test]
fn test_load_config() {
    let config = Config::load_from_file("tests/test_config.toml").unwrap();
    assert_eq!(config.app_name, "OmniRust");
    assert_eq!(config.port, 8080);
}
