mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use pingen2_sdk::error::PingenError;
use pingen2_sdk::webhook::IncomingWebhook;
use pingen2_sdk::WebhookEventCategory;
use serde_json::json;

#[tokio::test]
async fn test_webhooks_get_details() {
    let mut server = Server::new_async().await;
    let id = "wh000001-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/webhooks/{id}").as_str(),
        )
        .with_status(200)
        .with_header("x-request-id", "req1")
        .with_body(webhook_json(id))
        .create();
    let wh = Webhooks::new(ORG_ID, TOKEN, server.url());
    let r = wh.get_details(id, None).await.unwrap();
    assert_eq!(r.status_code, 200);
    assert_eq!(r.id, id);
}

#[tokio::test]
async fn test_webhooks_get_collection() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("GET", format!("/organisations/{ORG_ID}/webhooks").as_str())
        .with_status(200)
        .with_body(
            json!({"data": [{"id": "wh1", "type": "webhooks", "attributes": {}}]}).to_string(),
        )
        .create();
    let wh = Webhooks::new(ORG_ID, TOKEN, server.url());
    assert_eq!(wh.get_collection(None).await.unwrap().status_code, 200);
}

#[tokio::test]
async fn test_webhooks_create() {
    let mut server = Server::new_async().await;
    let id = "wh000002-xxxx-xxxx-xxxx-xxxxxxxxxx11";
    let _m = server
        .mock("POST", format!("/organisations/{ORG_ID}/webhooks").as_str())
        .with_status(201)
        .with_header("x-request-id", "req2")
        .with_body(webhook_json(id))
        .create();
    let wh = Webhooks::new(ORG_ID, TOKEN, server.url());
    let r = wh
        .create(WebhookEventCategory::Issues, "https://valid-url", "d09a")
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
    assert_eq!(r.id, id);
}

#[tokio::test]
async fn test_webhooks_delete() {
    let mut server = Server::new_async().await;
    let id = "wh000del-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock(
            "DELETE",
            format!("/organisations/{ORG_ID}/webhooks/{id}").as_str(),
        )
        .with_status(204)
        .create();
    let wh = Webhooks::new(ORG_ID, TOKEN, server.url());
    assert_eq!(wh.delete(id).await.unwrap().status_code, 204);
}

#[tokio::test]
async fn test_webhooks_delete_unauthorized() {
    let mut server = Server::new_async().await;
    let id = "wh000del-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock(
            "DELETE",
            format!("/organisations/{ORG_ID}/webhooks/{id}").as_str(),
        )
        .with_status(401)
        .with_body(access_denied_json())
        .create();
    let wh = Webhooks::new(ORG_ID, TOKEN, server.url());
    let err = wh.delete(id).await;
    assert!(matches!(err, Err(PingenError::Api { status: 401, .. })));
}

#[test]
fn test_webhook_valid_signature() {
    let payload =
        r#"{"data":{"type":"webhook_issues","id":"309a31e0-1abe-4034-8e7e-1fd473a802fd"}}"#;
    let secret = "webhook_test_secret123";
    let sig = {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(payload.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    };
    let event = IncomingWebhook::construct_event(payload, &sig, secret).unwrap();
    assert_eq!(
        event.data.as_ref().unwrap()["data"]["type"],
        "webhook_issues"
    );
}

#[test]
fn test_webhook_missing_signature() {
    let payload = r#"{"data":{"type":"webhook_issues"}}"#;
    let err = IncomingWebhook::construct_event(payload, "", "secret");
    assert!(matches!(err, Err(PingenError::WebhookSignature(_))));
}

#[test]
fn test_webhook_wrong_signature() {
    let payload = r#"{"data":{"type":"webhook_issues"}}"#;
    let err = IncomingWebhook::construct_event(
        payload,
        "wrongsig0000000000000000000000000000000000000000000000000000000000",
        "secret",
    );
    assert!(matches!(err, Err(PingenError::WebhookSignature(_))));
}

#[test]
fn test_webhook_valid_hex_but_incorrect_signature() {
    // A well-formed 32-byte hex string (correct length for HMAC-SHA256) that does
    // not match the payload/secret, exercising the constant-time comparison path
    // rather than the hex-decode-failure path.
    let payload = r#"{"data":{"type":"webhook_issues"}}"#;
    let bogus_signature = "0".repeat(64);
    let err = IncomingWebhook::construct_event(payload, &bogus_signature, "secret");
    assert!(matches!(err, Err(PingenError::WebhookSignature(_))));
}
