# ghcr

`ghcr` is a simple CLI tool for building and pushing Docker images to the GitHub Container Registry (GHCR). This tool also supports logging in to GHCR using Docker credentials securely via environment variables.

## Features

- **Build Docker Image**: Build a Docker image using the provided `Dockerfile` and context.
- **Push Docker Image**: Push the built Docker image to GitHub Container Registry (GHCR).
- **Login to GHCR**: Login to GHCR using your username and a token stored in an environment variable.

## Requirements

- Docker: You need Docker installed on your machine.
- GitHub Account: Required for authentication and pushing to GHCR.
- Rust: This tool is written in Rust, and you'll need to have Rust installed to build it locally. However, a pre-built binary can also be used.

## Installation

### Pre-built Binary

You can download the pre-built binary from the GitHub releases page.

### Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/khulnasoft-lab/ghcr.git
   cd ghcr
