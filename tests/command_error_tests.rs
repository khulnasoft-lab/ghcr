use ghcr::commands::{build, login, push, GhcrError};
use ghcr::config::{Auth, Config, Image};

#[test]
fn build_missing_tag() {
    let config = Config {
        image: Image {
            tag: "".to_string(),
            context: Some(".".to_string()),
        },
        auth: None,
    };
    let result = build(&config);
    assert!(matches!(result, Err(GhcrError::BuildError(_)) | Err(GhcrError::Io(_)) | Err(GhcrError::Other(_))));
}

#[test]
fn login_missing_token_env() {
    let auth = Auth {
        username: "user".to_string(),
        token_env: "SOME_FAKE_TOKEN_ENV".to_string(),
    };
    std::env::remove_var("SOME_FAKE_TOKEN_ENV");
    let result = login(&auth);
    assert!(matches!(result, Err(GhcrError::TokenEnvMissing(_))));
}
