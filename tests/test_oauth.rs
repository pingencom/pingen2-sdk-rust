use mockito::{Matcher, Server};
use pingen2_sdk::oauth::{OAuth, TokenManager};
use pingen2_sdk::PingenError;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_oauth_authorize_url_production() {
    let mut params = HashMap::new();
    params.insert(
        "redirect_uri".to_string(),
        "https://myapp.com/callback".to_string(),
    );
    let url = OAuth::authorize_url(false, Some("CLIENT_ID"), params).unwrap();
    assert!(url.contains("identity.pingen.com"));
    assert!(url.contains("client_id=CLIENT_ID"));
    assert!(url.contains("response_type=code"));
}

#[test]
fn test_oauth_authorize_url_staging() {
    let mut params = HashMap::new();
    params.insert(
        "redirect_uri".to_string(),
        "https://myapp.com/callback".to_string(),
    );
    let url = OAuth::authorize_url(true, Some("CLIENT_ID"), params).unwrap();
    assert!(url.contains("identity-staging.pingen.com"));
}

#[tokio::test]
async fn test_oauth_get_token() {
    let mut server = Server::new_async().await;
    let _m = server.mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(json!({"token_type": "Bearer", "expires_in": 43200, "access_token": "YOUR_ACCESS_TOKEN"}).to_string())
        .create();
    let v = OAuth::post_form(
        &format!("{}/auth/access-tokens", server.url()),
        &[("grant_type", "client_credentials")],
    )
    .await
    .unwrap();
    assert_eq!(v["access_token"], "YOUR_ACCESS_TOKEN");
}

#[tokio::test]
async fn test_oauth_post_form_errors_on_non_2xx_status() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(400)
        .with_body(json!({"error": "invalid_grant"}).to_string())
        .create();
    let result = OAuth::post_form(
        &format!("{}/auth/access-tokens", server.url()),
        &[("grant_type", "authorization_code"), ("code", "bad")],
    )
    .await;
    match result {
        Err(PingenError::Api { status, body }) => {
            assert_eq!(status, 400);
            assert!(body.contains("invalid_grant"));
        }
        other => panic!("expected PingenError::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn test_oauth_get_token_errors_on_non_2xx_status() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(401)
        .with_body(json!({"error": "invalid_client"}).to_string())
        .create();
    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());
    let result = OAuth::get_token(&server.url(), Some("cid"), Some("sec"), params).await;
    assert!(matches!(result, Err(PingenError::Api { status: 401, .. })));
}

#[test]
fn test_oauth_get_token_from_implicit() {
    let fragment = "access_token=abc123&expires_in=43200&token_type=Bearer";
    let result = OAuth::get_token_from_implicit(fragment);
    assert_eq!(result.get("access_token").unwrap(), "abc123");
    assert_eq!(result.get("expires_in").unwrap(), "43200");
    assert_eq!(result.get("token_type").unwrap(), "Bearer");
}

#[test]
fn test_oauth_get_token_from_implicit_empty() {
    let result = OAuth::get_token_from_implicit("");
    assert!(result.is_empty() || result.len() == 1);
}

#[test]
fn test_oauth_get_token_from_implicit_percent_decodes_values() {
    let fragment =
        "state=hello%20world&scope=read%3Awrite&redirect_uri=https%3A%2F%2Fapp.example%2Fcb";
    let result = OAuth::get_token_from_implicit(fragment);
    assert_eq!(result.get("state").unwrap(), "hello world");
    assert_eq!(result.get("scope").unwrap(), "read:write");
    assert_eq!(
        result.get("redirect_uri").unwrap(),
        "https://app.example/cb"
    );
}

#[tokio::test]
async fn test_oauth_post_form_via_get_token() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 43200, "access_token": "AT_123"})
                .to_string(),
        )
        .create();
    let v = pingen2_sdk::oauth::OAuth::post_form(
        &format!("{}/auth/access-tokens", server.url()),
        &[
            ("grant_type", "authorization_code"),
            ("client_id", "cid"),
            ("client_secret", "csecret"),
            ("code", "code123"),
            ("redirect_uri", "https://cb"),
        ],
    )
    .await
    .unwrap();
    assert_eq!(v["access_token"], "AT_123");
    assert_eq!(v["token_type"], "Bearer");
}

#[tokio::test]
async fn test_oauth_post_form_via_refresh() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(json!({"token_type": "Bearer", "access_token": "AT_REFRESHED"}).to_string())
        .create();
    let v = pingen2_sdk::oauth::OAuth::post_form(
        &format!("{}/auth/access-tokens", server.url()),
        &[
            ("grant_type", "refresh_token"),
            ("client_id", "cid"),
            ("client_secret", "csecret"),
            ("refresh_token", "rt_old"),
        ],
    )
    .await
    .unwrap();
    assert_eq!(v["access_token"], "AT_REFRESHED");
}

#[tokio::test]
async fn test_oauth_get_token_mock() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 3600, "access_token": "tok"}).to_string(),
        )
        .create();
    let v = OAuth::post_form(
        &format!("{}/auth/access-tokens", server.url()),
        &[
            ("grant_type", "authorization_code"),
            ("client_id", "cid"),
            ("client_secret", "cs"),
            ("code", "c"),
            ("redirect_uri", "https://cb"),
        ],
    )
    .await
    .unwrap();
    assert_eq!(v["access_token"], "tok");
}

#[tokio::test]
async fn test_oauth_refresh_token_mock() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 3600, "access_token": "refreshed"})
                .to_string(),
        )
        .create();
    let v = OAuth::post_form(
        &format!("{}/auth/access-tokens", server.url()),
        &[
            ("grant_type", "refresh_token"),
            ("client_id", "cid"),
            ("client_secret", "cs"),
            ("refresh_token", "rt"),
        ],
    )
    .await
    .unwrap();
    assert_eq!(v["access_token"], "refreshed");
}

#[tokio::test]
async fn test_oauth_client_credentials_mock() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 3600, "access_token": "cc_tok"})
                .to_string(),
        )
        .create();
    let v = OAuth::post_form(
        &format!("{}/auth/access-tokens", server.url()),
        &[
            ("grant_type", "client_credentials"),
            ("client_id", "cid"),
            ("client_secret", "cs"),
        ],
    )
    .await
    .unwrap();
    assert_eq!(v["access_token"], "cc_tok");
}

#[tokio::test]
async fn test_oauth_client_credentials_via_post_form() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 43200, "access_token": "CC_TOKEN"})
                .to_string(),
        )
        .create();
    let v = OAuth::post_form(
        &format!("{}/auth/access-tokens", server.url()),
        &[
            ("grant_type", "client_credentials"),
            ("client_id", "cid"),
            ("client_secret", "csecret"),
        ],
    )
    .await
    .unwrap();
    assert_eq!(v["access_token"], "CC_TOKEN");
}

#[test]
fn test_oauth_authorize_url_missing_client_id() {
    let result = OAuth::authorize_url(false, None, HashMap::new());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("No client_id provided"));
}

#[test]
fn test_oauth_authorize_url_client_id_in_params_overrides_default() {
    let mut params = HashMap::new();
    params.insert("client_id".to_string(), "from_params".to_string());
    let url = OAuth::authorize_url(false, Some("from_default"), params).unwrap();
    assert!(url.contains("client_id=from_params"));
}

#[test]
fn test_oauth_authorize_url_custom_response_type() {
    let mut params = HashMap::new();
    params.insert("response_type".to_string(), "token".to_string());
    let url = OAuth::authorize_url(false, Some("cid"), params).unwrap();
    assert!(url.contains("response_type=token"));
    assert!(!url.contains("response_type=code"));
}

#[tokio::test]
async fn test_oauth_get_token_with_defaults() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 3600, "access_token": "mocked"})
                .to_string(),
        )
        .create();

    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());
    let result = OAuth::get_token(&server.url(), Some("cid"), Some("sec"), params)
        .await
        .unwrap();
    assert_eq!(result["access_token"], "mocked");
    assert_eq!(result["token_type"], "Bearer");
}

#[tokio::test]
async fn test_oauth_get_token_fills_client_id_and_secret() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .match_body(Matcher::AllOf(vec![
            Matcher::UrlEncoded("client_id".into(), "default_cid".into()),
            Matcher::UrlEncoded("client_secret".into(), "default_sec".into()),
            Matcher::UrlEncoded("grant_type".into(), "authorization_code".into()),
            Matcher::UrlEncoded("code".into(), "authcode".into()),
        ]))
        .with_status(200)
        .with_body(json!({"access_token": "ok"}).to_string())
        .expect(1)
        .create();

    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "authorization_code".to_string());
    params.insert("code".to_string(), "authcode".to_string());
    let result = OAuth::get_token(
        &server.url(),
        Some("default_cid"),
        Some("default_sec"),
        params,
    )
    .await
    .unwrap();
    assert_eq!(result["access_token"], "ok");
    _m.assert_async().await;
}

#[tokio::test]
async fn test_oauth_get_token_params_override_defaults() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .match_body(Matcher::AllOf(vec![
            Matcher::UrlEncoded("client_id".into(), "from_params".into()),
            Matcher::UrlEncoded("client_secret".into(), "from_params_sec".into()),
        ]))
        .with_status(200)
        .with_body(json!({"access_token": "overridden"}).to_string())
        .expect(1)
        .create();

    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());
    params.insert("client_id".to_string(), "from_params".to_string());
    params.insert("client_secret".to_string(), "from_params_sec".to_string());
    let result = OAuth::get_token(&server.url(), Some("ignored"), Some("ignored"), params)
        .await
        .unwrap();
    assert_eq!(result["access_token"], "overridden");
    _m.assert_async().await;
}

#[tokio::test]
async fn test_oauth_get_token_missing_client_id() {
    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());
    let result = OAuth::get_token("http://unused", None, Some("sec"), params).await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No client_id provided"));
}

#[tokio::test]
async fn test_oauth_get_token_missing_client_secret() {
    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());
    let result = OAuth::get_token("http://unused", Some("cid"), None, params).await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No client_secret provided"));
}

#[tokio::test]
async fn test_token_manager_reuses_valid_token() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 3600, "access_token": "first"})
                .to_string(),
        )
        .expect(1)
        .create();

    let manager = TokenManager::new(
        server.url(),
        Some("cid".to_string()),
        Some("sec".to_string()),
    );

    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());

    let first = manager.get_token(params.clone()).await.unwrap();
    let second = manager.get_token(params).await.unwrap();

    assert_eq!(first, "first");
    assert_eq!(second, "first");
    _m.assert_async().await;
}

#[tokio::test]
async fn test_token_manager_refreshes_expired_token() {
    let mut server = Server::new_async().await;
    let _m1 = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 0, "access_token": "expired"}).to_string(),
        )
        .expect(1)
        .create();

    let manager = TokenManager::new(
        server.url(),
        Some("cid".to_string()),
        Some("sec".to_string()),
    );
    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());

    let first = manager.get_token(params.clone()).await.unwrap();
    assert_eq!(first, "expired");
    _m1.assert_async().await;

    let _m2 = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 3600, "access_token": "renewed"})
                .to_string(),
        )
        .expect(1)
        .create();

    let second = manager.get_token(params).await.unwrap();
    assert_eq!(second, "renewed");
    _m2.assert_async().await;
}

#[tokio::test]
async fn test_token_manager_invalidate_forces_refresh() {
    let mut server = Server::new_async().await;
    let _m1 = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 3600, "access_token": "one"}).to_string(),
        )
        .expect(1)
        .create();

    let manager = TokenManager::new(
        server.url(),
        Some("cid".to_string()),
        Some("sec".to_string()),
    );
    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());

    let first = manager.get_token(params.clone()).await.unwrap();
    assert_eq!(first, "one");
    _m1.assert_async().await;

    manager.invalidate().await;

    let _m2 = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 3600, "access_token": "two"}).to_string(),
        )
        .expect(1)
        .create();

    let second = manager.get_token(params).await.unwrap();
    assert_eq!(second, "two");
    _m2.assert_async().await;
}
