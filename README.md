# ghcr

[![CI](https://github.com/khulnasoft-lab/ghcr/actions/workflows/ci.yml/badge.svg)](https://github.com/khulnasoft-lab/ghcr/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/ghcr.svg)](https://crates.io/crates/ghcr)
[![Docs.rs](https://docs.rs/ghcr/badge.svg)](https://docs.rs/ghcr)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)

`ghcr` is a CLI tool to build and publish Docker images to GitHub Container Registry (GHCR). This tool also supports logging in to GHCR using Docker credentials securely via environment variables.

---

## Features
- **Modularized Rust code** for easy maintenance and extensibility
- **Robust error handling** and informative logging
- **Modern CI/CD** with matrix Rust testing and Docker build/push
- **Comprehensive tests** for config, commands, and integration scenarios
- **Build Docker Image**: Build a Docker image using the provided `Dockerfile` and context.
- **Push Docker Image**: Push the built Docker image to GitHub Container Registry (GHCR).
- **Login to GHCR**: Login to GHCR using your username and a token stored in an environment variable.

---

## GitHub Action Usage

[![GitHub Marketplace](https://img.shields.io/badge/marketplace-ghcr--action-blue?logo=github)](https://github.com/marketplace/actions/ghcr)

You can use this project as a GitHub Action to build, login, and push Docker images to GHCR in your CI workflows.

**Basic Example:**
```yaml
- name: Build and push Docker image
  uses: khulnasoft-lab/ghcr@v1
  with:
    command: build
    config: ./ghcr.toml
    token: ${{ secrets.GHCR_TOKEN }}
```

**Advanced Multi-Step Example:**
```yaml
jobs:
  build-and-push:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Login to GHCR
        uses: khulnasoft-lab/ghcr@v1
        with:
          command: login
          config: ./ghcr.toml
          token: ${{ secrets.GHCR_TOKEN }}
      - name: Build Docker image
        uses: khulnasoft-lab/ghcr@v1
        with:
          command: build
          config: ./ghcr.toml
      - name: Push Docker image
        uses: khulnasoft-lab/ghcr@v1
        with:
          command: push
          config: ./ghcr.toml
          token: ${{ secrets.GHCR_TOKEN }}
```

- Pin to a specific version, e.g., `@v1`, for stability in your workflows.
- See the [action.yml](./action.yml) for all available inputs.

---

## Versioning & Releases

To use a stable version of this action, pin to a release tag (e.g., `@v1` or `@v1.0.0`).

To publish a new release:
1. Commit all changes to `main`.
2. Tag a release:
   ```sh
   git tag v1.0.0
   git push --tags
   ```
3. The action will appear on the GitHub Marketplace and can be referenced as `khulnasoft-lab/ghcr@v1.0.0` in workflows.

---

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
   ```
2. Build the binary:
   ```bash
   cargo build --release
   ```
3. The binary will be in `target/release/ghcr`.

## Configuration

Create a `ghcr.toml` file in your project root. Example:

```toml
[package]
name = "ghcr"
version = "0.1.0"
description = "Build and publish Docker images to GitHub Container Registry"
repository = "https://github.com/khulnasoft-lab/ghcr"

[image]
tag = "ghcr.io/khulnasoft-lab/ghcr:latest"
context = "."

[auth]
username = "khulnasoft-lab"
token_env = "GHCR_TOKEN"  # Set this environment variable with your GHCR token
```

## Usage Examples

### Build Docker Image
```bash
RUST_LOG=info ./target/release/ghcr build
```

### Push Docker Image
```bash
RUST_LOG=info ./target/release/ghcr push
```

### Login to GHCR
```bash
export GHCR_TOKEN=your_token_here
RUST_LOG=info ./target/release/ghcr login
```

## .dockerignore
A recommended `.dockerignore` is included to keep your Docker build context clean and secure. Edit as needed for your project.

## Troubleshooting & Common Errors

- **Docker Daemon Not Running:**
  - Error: `Cannot connect to the Docker daemon...`
  - Solution: Ensure Docker is installed and running (`docker info`).
- **Token Environment Variable Not Set:**
  - Error: `Token environment variable 'GHCR_TOKEN' not set`
  - Solution: Export the variable before running `login`.
- **Invalid TOML Format:**
  - Error: `Invalid TOML format: ...`
  - Solution: Check your `ghcr.toml` for syntax errors.
- **Permission Issues:**
  - Ensure you have permission to access the Docker socket and to push images to GHCR.

## Security Best Practices
- Use environment variables for secrets (never hardcode tokens).
- Use the provided `.dockerignore` to avoid leaking sensitive files into your Docker images.
- Run containers as a non-root user (see the provided Dockerfile).

## Testing
- Add unit and integration tests for command logic and configuration parsing (see `src/main.rs`).
- Example test frameworks: [`assert_cmd`](https://docs.rs/assert_cmd), [`tempfile`](https://docs.rs/tempfile).

## CI/CD
- Add workflows in `.github/workflows/` for linting, testing, and building Docker images.
- Example: GitHub Actions for Rust and Docker.

## License
MIT OR Apache-2.0
