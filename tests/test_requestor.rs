mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::requestor::ApiRequestor;
use pingen2_sdk::api::*;
use pingen2_sdk::oauth::TokenManager;
use pingen2_sdk::TokenProvider;
use serde_json::json;
use std::collections::HashMap;

#[tokio::test]
async fn test_requestor_get_with_params() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("GET", "/organisations/org1/deliveries/letters")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page[number]".into(), "2".into()),
            mockito::Matcher::UrlEncoded("page[size]".into(), "10".into()),
        ]))
        .with_status(200)
        .with_body(json!({"data": []}).to_string())
        .create();
    let letters = Letters::new("org1", TOKEN, server.url());
    let mut params = HashMap::new();
    params.insert("page[number]".to_string(), "2".to_string());
    params.insert("page[size]".to_string(), "10".to_string());
    assert_eq!(
        letters
            .get_collection(Some(&params))
            .await
            .unwrap()
            .status_code,
        200
    );
}

#[tokio::test]
async fn test_requestor_download() {
    let mut server = Server::new_async().await;
    let id = "stream01-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let binary_body: &[u8] = b"\x00\x01\xFF\xFEbinary-data-here";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/letters/{id}/file").as_str(),
        )
        .with_status(200)
        .with_body(binary_body)
        .create();
    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let content = letters.get_file(id).await.unwrap();
    assert_eq!(content, binary_body);
}

#[tokio::test]
async fn test_file_upload_no_data_error() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("GET", "/file-upload")
        .with_status(200)
        .with_body("")
        .create();
    let req = ApiRequestor::new(TOKEN, server.url());
    let fu = pingen2_sdk::api::file_upload::FileUpload::new(&req);
    let result = fu.request_file_upload().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_requestor_managed_token_auto_refresh() {
    let mut server = Server::new_async().await;
    // expires_in 0 means every cached token is already expired, so each API call
    // must transparently fetch a fresh token from the token endpoint.
    let token_mock = server
        .mock("POST", "/auth/access-tokens")
        .with_status(200)
        .with_body(
            json!({"token_type": "Bearer", "expires_in": 0, "access_token": "managed_tok"})
                .to_string(),
        )
        .expect_at_least(2)
        .create();
    let letters_mock = server
        .mock("GET", "/organisations/org1/deliveries/letters")
        .match_header("authorization", "Bearer managed_tok")
        .with_status(200)
        .with_body(json!({"data": []}).to_string())
        .expect(2)
        .create();

    let manager = TokenManager::new(
        server.url(),
        Some("cid".to_string()),
        Some("sec".to_string()),
    );
    let letters = Letters::new("org1", manager, server.url());

    let first = letters.get_collection(None).await.unwrap();
    let second = letters.get_collection(None).await.unwrap();

    assert_eq!(first.status_code, 200);
    assert_eq!(second.status_code, 200);
    token_mock.assert_async().await;
    letters_mock.assert_async().await;
}

#[tokio::test]
async fn test_token_provider_static_from_str() {
    let provider = TokenProvider::from("tok");
    assert_eq!(provider.access_token().await.unwrap(), "tok");
}

#[tokio::test]
async fn test_requestor_build_url_with_bad_base() {
    let req = ApiRequestor::new(TOKEN, "not-a-valid-url");
    let mut params = HashMap::new();
    params.insert("key".to_string(), "val".to_string());
    let result = req.get("/path", Some(&params)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_token_provider_all_conversions() {
    use std::sync::Arc;

    // From<&str> / From<String> / From<&String> resolve to the static token.
    let p: TokenProvider = "a".into();
    assert_eq!(p.access_token().await.unwrap(), "a");
    let p: TokenProvider = String::from("b").into();
    assert_eq!(p.access_token().await.unwrap(), "b");
    let owned = String::from("c");
    let p: TokenProvider = (&owned).into();
    assert_eq!(p.access_token().await.unwrap(), "c");

    // From<TokenManager> / From<Arc<TokenManager>> / From<&Arc<TokenManager>>
    // build the managed variant (no network needed to construct).
    let _p: TokenProvider = TokenManager::new("http://unused", None, None).into();
    let arc = Arc::new(TokenManager::new("http://unused", None, None));
    let _p: TokenProvider = arc.clone().into();
    let _p: TokenProvider = (&arc).into();
}

#[tokio::test]
async fn test_requestor_invalid_token_header_errors() {
    // A token containing a control character cannot be encoded into an
    // Authorization header, so the request fails before it is sent.
    let req = ApiRequestor::new("bad\ntoken", "http://localhost");
    let result = req.get("/x", None).await;
    assert!(matches!(
        result,
        Err(pingen2_sdk::PingenError::Authentication(_))
    ));
}

#[tokio::test]
async fn test_requestor_download_error_status() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("GET", "/x")
        .with_status(404)
        .with_body("nope")
        .create();
    let req = ApiRequestor::new(TOKEN, server.url());
    assert!(req.download("/x").await.is_err());
}

#[tokio::test]
async fn test_requestor_put_file_error_status() {
    let mut server = Server::new_async().await;
    let _m = server.mock("PUT", "/up").with_status(500).create();
    let req = ApiRequestor::new(TOKEN, server.url());
    let url = format!("{}/up", server.url());
    assert!(req.put_file(&url, &fixture_pdf()).await.is_err());
}

#[test]
fn test_token_provider_debug_managed_redacts() {
    use std::sync::Arc;
    let manager = Arc::new(TokenManager::client_credentials(
        "http://unused",
        "cid",
        "SUPER_SECRET",
        None,
    ));
    let provider: TokenProvider = manager.into();
    let dbg = format!("{provider:?}");
    assert!(dbg.contains("Managed"));
    assert!(
        !dbg.contains("SUPER_SECRET"),
        "managed provider must not leak the client secret via Debug: {dbg}"
    );
    assert!(dbg.contains("[redacted]"));
}
