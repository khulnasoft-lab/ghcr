//! Command implementations for the ghcr CLI tool.

use crate::config::{Auth, Config};
use log::{error, info};
use std::io::Write;
use std::process::Command;

/// Builds the Docker image as specified in the config.
pub fn build(config: &Config) -> Result<(), String> {
    let context = config
        .image
        .context
        .clone()
        .unwrap_or_else(|| ".".to_string());
    info!("Building Docker image: {}", config.image.tag);
    let status = Command::new("docker")
        .args(["build", "-t", &config.image.tag, &context])
        .status()
        .map_err(|e| format!("Failed to run docker build: {}", e))?;
    if !status.success() {
        error!("Docker build failed with status: {}", status);
        return Err("Docker build failed".to_string());
    }
    Ok(())
}

/// Pushes the Docker image to the registry as specified in the config.
pub fn push(config: &Config) -> Result<(), String> {
    info!("Pushing Docker image: {}", config.image.tag);
    let status = Command::new("docker")
        .args(["push", &config.image.tag])
        .status()
        .map_err(|e| format!("Failed to run docker push: {}", e))?;
    if !status.success() {
        error!("Docker push failed with status: {}", status);
        return Err("Docker push failed".to_string());
    }
    Ok(())
}

/// Logs in to GHCR using the provided authentication config.
pub fn login(auth: &Auth) -> Result<(), String> {
    let token = std::env::var(&auth.token_env)
        .map_err(|_| format!("Token environment variable '{}' not set", auth.token_env))?;
    info!("Logging in to GHCR as {}", auth.username);
    let mut child = Command::new("docker")
        .args(["login", "ghcr.io", "-u", &auth.username, "--password-stdin"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start docker login: {}", e))?;
    child
        .stdin
        .as_mut()
        .ok_or("Failed to open stdin for docker login".to_string())?
        .write_all(token.as_bytes())
        .map_err(|e| format!("Failed to write token to stdin: {}", e))?;
    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait on docker login: {}", e))?;
    if !status.success() {
        error!("Docker login failed with status: {}", status);
        return Err("Docker login failed".to_string());
    }
    Ok(())
}
