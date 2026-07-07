mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use serde_json::json;

#[tokio::test]
async fn test_letter_events_get_collection() {
    let mut server = Server::new_async().await;
    let lid = "letter01-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/letters/{lid}/events").as_str(),
        )
        .with_status(200)
        .with_body(
            json!({"data": [{"id": "ev001", "type": "letter_events", "attributes": {}}]})
                .to_string(),
        )
        .create();
    let le = LetterEvents::new(ORG_ID, TOKEN, server.url());
    assert_eq!(le.get_collection(lid, None).await.unwrap().status_code, 200);
}

#[tokio::test]
async fn test_letter_events_issues() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/letters/events/issues").as_str(),
        )
        .with_status(200)
        .with_body(json!({"data": []}).to_string())
        .create();
    let le = LetterEvents::new(ORG_ID, TOKEN, server.url());
    assert_eq!(
        le.get_issue_collection(None).await.unwrap().status_code,
        200
    );
}

#[tokio::test]
async fn test_letter_events_undeliverable() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/letters/events/undeliverable").as_str(),
        )
        .with_status(200)
        .with_body(json!({"data": []}).to_string())
        .create();
    let le = LetterEvents::new(ORG_ID, TOKEN, server.url());
    assert_eq!(
        le.get_undeliverable_collection(None)
            .await
            .unwrap()
            .status_code,
        200
    );
}

#[tokio::test]
async fn test_letter_events_delivered() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/letters/events/delivered").as_str(),
        )
        .with_status(200)
        .with_body(json!({"data": []}).to_string())
        .create();
    let le = LetterEvents::new(ORG_ID, TOKEN, server.url());
    assert_eq!(
        le.get_delivered_collection(None).await.unwrap().status_code,
        200
    );
}

#[tokio::test]
async fn test_letter_events_sent() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/letters/events/sent").as_str(),
        )
        .with_status(200)
        .with_body(json!({"data": []}).to_string())
        .create();
    let le = LetterEvents::new(ORG_ID, TOKEN, server.url());
    assert_eq!(le.get_sent_collection(None).await.unwrap().status_code, 200);
}
