use ghcr::config::{load_config, Config};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn parse_valid_toml() {
    let toml = r#"
[image]
tag = "test:latest"
context = "."
[auth]
username = "user"
token_env = "TOKEN"
"#;
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", toml).unwrap();
    let path = file.path();
    let toml_str = std::fs::read_to_string(path).unwrap();
    let config: Config = toml::from_str(&toml_str).unwrap();
    assert_eq!(config.image.tag, "test:latest");
    assert_eq!(config.auth.unwrap().username, "user");
}

#[test]
fn parse_invalid_toml() {
    let toml = r#"[image] tag = "missing_quote
[auth] username = user token_env = TOKEN"#;
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", toml).unwrap();
    let path = file.path();
    let toml_str = std::fs::read_to_string(path).unwrap();
    let result: Result<Config, _> = toml::from_str(&toml_str);
    assert!(result.is_err());
}
