use ghcr::commands::{build, login, push};
use ghcr::config::{Auth, Config, Image};
use std::env;

/// Integration test for building and pushing an image to GHCR.
/// This test requires Docker and valid credentials in the environment.
#[test]
#[ignore] // Run manually: cargo test --test docker_success -- --ignored
fn build_and_push_success() {
    // These should be set in your environment for the test to work
    let username = env::var("GHCR_TEST_USER").expect("GHCR_TEST_USER not set");
    let token = env::var("GHCR_TEST_TOKEN").expect("GHCR_TEST_TOKEN not set");
    let image_tag = format!("ghcr.io/{}/ghcr-test:ci", username);
    env::set_var("GHCR_TEST_TOKEN_ENV", &token);

    let config = Config {
        image: Image {
            tag: image_tag.clone(),
            context: Some(".".into()),
        },
        auth: Some(Auth {
            username: username.clone(),
            token_env: "GHCR_TEST_TOKEN_ENV".into(),
        }),
    };
    // Login
    login(config.auth.as_ref().unwrap()).expect("Login failed");
    // Build
    build(&config).expect("Build failed");
    // Push
    push(&config).expect("Push failed");
}
