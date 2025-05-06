mod commands;
mod config;

use clap::{Parser, Subcommand};
use commands::*;
use config::load_config;
use log::error;

fn init_logging() {
    env_logger::init();
}

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

fn run() -> Result<(), commands::GhcrError> {
    let cli = Cli::parse();
    let config = load_config()?;
    match cli.command {
        Commands::Build => build(&config),
        Commands::Push => push(&config),
        Commands::Login => {
            let auth = config.auth.as_ref().ok_or(commands::GhcrError::Other(
                "No [auth] section in ghcr.toml. Please add GHCR credentials.".to_string(),
            ))?;
            login(auth)
        }
    }
}

fn main() {
    init_logging();
    if let Err(e) = run() {
        error!("{e}");
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
