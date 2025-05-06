//! Configuration structures and loading logic for the ghcr CLI tool.

use serde::Deserialize;
use std::fs;

/// Root configuration struct loaded from ghcr.toml.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// Image build/push settings.
    pub image: Image,
    /// Optional authentication settings.
    pub auth: Option<Auth>,
}

/// Docker image configuration.
#[derive(Deserialize, Debug)]
pub struct Image {
    /// Tag for the Docker image (e.g., ghcr.io/user/repo:tag).
    pub tag: String,
    /// Optional build context directory.
    pub context: Option<String>,
}

/// Authentication configuration for GHCR.
#[derive(Deserialize, Debug)]
pub struct Auth {
    /// Username for GHCR.
    pub username: String,
    /// Name of the environment variable holding the token.
    pub token_env: String,
}

/// Loads the configuration from ghcr.toml in the current directory.
/// Returns a Config struct on success, or a String error message.
pub fn load_config() -> Result<Config, String> {
    let toml_str =
        fs::read_to_string("ghcr.toml").map_err(|e| format!("Failed to read ghcr.toml: {}", e))?;
    toml::from_str(&toml_str).map_err(|e| format!("Invalid TOML format: {}", e))
}
