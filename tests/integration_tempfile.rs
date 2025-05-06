use ghcr::config::load_config;
use std::env;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn load_config_from_tempfile() {
    let toml = r#"
[image]
tag = "tempfile:test"
context = "/tmp"
[auth]
username = "tempuser"
token_env = "TEMP_TOKEN"
"#;
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", toml).unwrap();
    let path = file.path();
    // Temporarily change working directory to the temp file's directory
    let orig_dir = env::current_dir().unwrap();
    env::set_current_dir(path.parent().unwrap()).unwrap();
    std::fs::copy(path, "ghcr.toml").unwrap();
    let config = load_config().unwrap();
    assert_eq!(config.image.tag, "tempfile:test");
    assert_eq!(config.auth.unwrap().username, "tempuser");
    std::fs::remove_file("ghcr.toml").unwrap();
    env::set_current_dir(orig_dir).unwrap();
}
