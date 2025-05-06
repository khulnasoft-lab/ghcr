use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn build_command_fails_without_docker() {
    let mut cmd = Command::cargo_bin("ghcr").unwrap();
    cmd.arg("build");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Docker build failed"));
}

#[test]
fn login_command_missing_token_env() {
    let mut cmd = Command::cargo_bin("ghcr").unwrap();
    cmd.arg("login");
    // Unset the token env var for this test
    cmd.env_remove("GHCR_TOKEN");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Token environment variable"));
}
