//! OAuth integration tests against the Pingen staging environment.
//!
//! Verifies the stateful [`TokenManager`] token source end to end: a
//! client-credentials token can be obtained and used, it is reused while still
//! valid, and after invalidation the next call transparently fetches a fresh,
//! working token.
//!
//! These tests make real HTTP calls and require valid staging credentials in
//! `.env` (see `tests/integration/common.rs`). They are `#[ignore]`d so the
//! normal unit-test run and CI skip them. Run them explicitly with:
//!
//! ```text
//! cargo test --test integration_pingen --test integration_oauth -- --ignored --test-threads=1
//! ```

mod common;

use pingen2_sdk::{Organisations, TokenManager};
use std::sync::Arc;

fn build_token_manager(credentials: &common::Credentials) -> Arc<TokenManager> {
    Arc::new(TokenManager::client_credentials(
        common::api_base(credentials),
        credentials.client_id.clone(),
        credentials.client_secret.clone(),
        Some(common::SCOPE),
    ))
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn token_can_be_obtained_and_used() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let manager = build_token_manager(&credentials);

    let token = manager
        .get_access_token()
        .await
        .expect("token request must succeed");
    assert!(!token.is_empty(), "Token request must return a token");

    let api_base = common::api_base(&credentials);
    let organisations = Organisations::new(manager.clone(), api_base);
    let response = organisations
        .get_collection(None)
        .await
        .expect("listing organisations with the managed token must succeed");
    assert_eq!(response.status_code, 200);
    assert!(
        !response.data.is_empty(),
        "Expected at least one organisation"
    );
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn token_is_reused_while_valid() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let manager = build_token_manager(&credentials);

    let first = manager
        .get_access_token()
        .await
        .expect("first token request must succeed");
    let second = manager
        .get_access_token()
        .await
        .expect("second token request must succeed");

    assert_eq!(first, second, "A still-valid token must be reused");
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn invalidated_token_is_refreshed() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let manager = build_token_manager(&credentials);
    let api_base = common::api_base(&credentials);
    let organisations = Organisations::new(manager.clone(), api_base);

    // Initial call acquires the first token and proves it works.
    let response = organisations
        .get_collection(None)
        .await
        .expect("initial call with the managed token must succeed");
    assert_eq!(response.status_code, 200);

    // Discard the cached token, guaranteeing a refresh on the next request.
    manager.invalidate().await;

    // The next resource call must transparently fetch a working token again.
    let response = organisations
        .get_collection(None)
        .await
        .expect("call after invalidation must succeed with a refreshed token");
    assert_eq!(response.status_code, 200);

    let refreshed = manager
        .get_access_token()
        .await
        .expect("token request after invalidation must succeed");
    assert!(
        !refreshed.is_empty(),
        "A working token must be available after invalidation"
    );
}
