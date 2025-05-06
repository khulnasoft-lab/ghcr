//! Command implementations for the ghcr CLI tool.

use crate::config::{Auth, Config};
use log::{error, info};
use std::io::Write;
use std::process::Command;
use thiserror::Error;

/// Rich error type for all ghcr CLI operations.
#[derive(Error, Debug)]
pub enum GhcrError {
    /// Error during Docker build
    #[error("Docker build failed: {0}")]
    BuildError(String),
    /// Error during Docker push
    #[error("Docker push failed: {0}")]
    PushError(String),
    /// Error during Docker login
    #[error("Docker login failed: {0}")]
    LoginError(String),
    /// Missing required environment variable for token
    #[error("Token environment variable '{0}' not set")]
    TokenEnvMissing(String),
    /// Error reading or parsing config file
    #[error("Config error: {0}")]
    ConfigError(String),
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Any other error
    #[error("Other error: {0}")]
    Other(String),
}
// Note: thiserror automatically implements Display for user-friendly output.

/// Builds the Docker image as specified in the config.
pub fn build(config: &Config) -> Result<(), GhcrError> {
    let context = config
        .image
        .context
        .clone()
        .unwrap_or_else(|| ".".to_string());
    info!("Building Docker image: {}", config.image.tag);
    let status = Command::new("docker")
        .args(["build", "-t", &config.image.tag, &context])
        .status()
        .map_err(GhcrError::Io)?;
    if !status.success() {
        error!("Docker build failed with status: {status}");
        return Err(GhcrError::BuildError(status.to_string()));
    }
    Ok(())
}

/// Pushes the Docker image to the registry as specified in the config.
pub fn push(config: &Config) -> Result<(), GhcrError> {
    info!("Pushing Docker image: {}", config.image.tag);
    let status = Command::new("docker")
        .args(["push", &config.image.tag])
        .status()
        .map_err(GhcrError::Io)?;
    if !status.success() {
        error!("Docker push failed with status: {status}");
        return Err(GhcrError::PushError(status.to_string()));
    }
    Ok(())
}

/// Logs in to GHCR using the provided authentication config.
pub fn login(auth: &Auth) -> Result<(), GhcrError> {
    let token = std::env::var(&auth.token_env)
        .map_err(|_| GhcrError::TokenEnvMissing(auth.token_env.clone()))?;
    info!("Logging in to GHCR as {}", auth.username);
    let mut child = Command::new("docker")
        .args(["login", "ghcr.io", "-u", &auth.username, "--password-stdin"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(GhcrError::Io)?;
    child
        .stdin
        .as_mut()
        .ok_or(GhcrError::LoginError(
            "Failed to open stdin for docker login".to_string(),
        ))?
        .write_all(token.as_bytes())
        .map_err(GhcrError::Io)?;
    let status = child.wait().map_err(GhcrError::Io)?;
    if !status.success() {
        error!("Docker login failed with status: {status}");
        return Err(GhcrError::LoginError(status.to_string()));
    }
    Ok(())
}
