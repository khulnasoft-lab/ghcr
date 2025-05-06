use ghcr::commands::GhcrError;
use ghcr::config::Config;
use std::fs;
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
    let result = toml::from_str::<Config>(&toml_str);
    assert!(result.is_err()); // TOML parse error
                              // Now test our loader, which should return GhcrError::ConfigError
                              // Write the invalid TOML to a temp file and try to load it as config
    let tmp_path = path.to_str().unwrap();
    // Temporarily rename the file to 'ghcr.toml' so load_config() reads it
    let backup = if fs::metadata("ghcr.toml").is_ok() {
        Some(fs::read_to_string("ghcr.toml").unwrap())
    } else {
        None
    };
    fs::copy(tmp_path, "ghcr.toml").unwrap();
    let result = ghcr::config::load_config();
    assert!(matches!(result, Err(GhcrError::ConfigError(_))));
    // Restore original ghcr.toml if existed
    if let Some(contents) = backup {
        fs::write("ghcr.toml", contents).unwrap();
    } else {
        let _ = fs::remove_file("ghcr.toml");
    }
}
