use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{fs, io::Write, process::Command};

/// ghcr: A CLI tool to build and publish Docker images to GitHub Container Registry (GHCR).
#[derive(Parser)]
#[command(name = "ghcr")]
#[command(version = "0.1.0")]
#[command(about = "Build and publish Docker images to GHCR", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the Docker image
    Build,

    /// Push the Docker image
    Push,

    /// Login to GHCR using Docker credentials
    Login,
}

#[derive(Deserialize)]
struct Config {
    image: Image,
    auth: Option<Auth>,
}

#[derive(Deserialize)]
struct Image {
    tag: String,
    context: Option<String>,
}

#[derive(Deserialize)]
struct Auth {
    username: String,
    token_env: String,
}

/// Load configuration from the `ghcr.toml` file
fn load_config() -> Config {
    let toml_str = fs::read_to_string("ghcr.toml").expect("Failed to read ghcr.toml");
    toml::from_str(&toml_str).expect("Invalid TOML format")
}

fn main() {
    let cli = Cli::parse();
    let config = load_config();

    match cli.command {
        Commands::Build => {
            let context = config.image.context.unwrap_or_else(|| ".".to_string());
            println!("Building Docker image: {}", config.image.tag);
            let status = Command::new("docker")
                .args(["build", "-t", &config.image.tag, &context])
                .status()
                .expect("Failed to build Docker image");
            std::process::exit(status.code().unwrap_or(1));
        }

        Commands::Push => {
            println!("Pushing Docker image: {}", config.image.tag);
            let status = Command::new("docker")
                .args(["push", &config.image.tag])
                .status()
                .expect("Failed to push Docker image");
            std::process::exit(status.code().unwrap_or(1));
        }

        Commands::Login => {
            if let Some(auth) = config.auth {
                let token = std::env::var(&auth.token_env)
                    .expect("Token environment variable not set");
                println!("Logging in to GHCR as {}", auth.username);

                let mut child = Command::new("docker")
                    .args(["login", "ghcr.io", "-u", &auth.username, "--password-stdin"])
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .expect("Failed to start docker login");

                child
                    .stdin
                    .as_mut()
                    .expect("Failed to open stdin")
                    .write_all(token.as_bytes())
                    .expect("Failed to write token to stdin");

                let status = child.wait().expect("Failed to wait on docker login");
                std::process::exit(status.code().unwrap_or(1));
            } else {
                eprintln!("No [auth] section in ghcr.toml. Please add GHCR credentials.");
                std::process::exit(1);
            }
        }
    }
}
