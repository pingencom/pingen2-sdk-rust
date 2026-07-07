mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use pingen2_sdk::{EmailMetaData, EmailRelationships, PresetRelationship};
use serde_json::json;

fn email_json(id: &str) -> String {
    json!({
        "data": { "id": id, "type": "emails", "attributes": {
            "status": "string", "file_original_name": "lorem.pdf", "file_pages": 2,
            "recipient_identifier": "info@acme.com",
            "price_currency": "CHF", "price_value": 1.25,
            "source": "api",
            "submitted_at": "2021-11-19T09:42:48+0100",
            "created_at": "2020-11-19T09:42:48+0100",
            "updated_at": "2020-11-19T09:42:48+0100"
        }, "relationships": {
            "organisation": {
                "data": { "id": "org-1", "type": "organisations" },
                "links": { "related": "https://api/orgs/org-1" }
            },
            "events": {
                "links": { "related": { "href": "https://api/events", "meta": { "count": 0 }}}
            }
        }}
    })
    .to_string()
}

#[tokio::test]
async fn test_emails_get_details() {
    let mut server = Server::new_async().await;
    let id = "email001-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/emails/{id}").as_str(),
        )
        .with_status(200)
        .with_body(email_json(id))
        .create();
    let em = Emails::new(ORG_ID, TOKEN, server.url());
    let r = em.get_details(id, None).await.unwrap();
    assert_eq!(r.status_code, 200);
    assert_eq!(r.id, id);
    assert_eq!(r.attributes.status.as_deref(), Some("string"));
    assert_eq!(
        r.attributes.recipient_identifier.as_deref(),
        Some("info@acme.com")
    );
    assert_eq!(r.attributes.price_currency.as_deref(), Some("CHF"));

    let rels: EmailRelationships = r.typed_relationships().unwrap();
    assert_eq!(
        rels.organisation
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .id,
        "org-1"
    );
}

#[tokio::test]
async fn test_emails_get_collection() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/emails").as_str(),
        )
        .with_status(200)
        .with_body(json!({"data": []}).to_string())
        .create();
    let em = Emails::new(ORG_ID, TOKEN, server.url());
    assert_eq!(em.get_collection(None).await.unwrap().status_code, 200);
}

#[tokio::test]
async fn test_emails_create() {
    let mut server = Server::new_async().await;
    let id = "email002-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/emails").as_str(),
        )
        .with_status(201)
        .with_body(
            json!({"data": {"id": id, "type": "emails", "attributes": {"status": "string"}}})
                .to_string(),
        )
        .create();
    let em = Emails::new(ORG_ID, TOKEN, server.url());
    assert_eq!(
        em.create("https://s3.ex/file", "$sig", "lorem.pdf", false, None, None)
            .await
            .unwrap()
            .status_code,
        201
    );
}

#[tokio::test]
async fn test_emails_upload_and_create() {
    let mut server = Server::new_async().await;
    let (_um, _sm) = stub_file_upload(&mut server);
    let id = "email003-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _pm = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/emails").as_str(),
        )
        .with_status(201)
        .with_body(
            json!({"data": {"id": id, "type": "emails", "attributes": {"status": "string"}}})
                .to_string(),
        )
        .create();
    let em = Emails::new(ORG_ID, TOKEN, server.url());
    assert_eq!(
        em.upload_and_create(&fixture_pdf(), "lorem.pdf", false, None, None)
            .await
            .unwrap()
            .status_code,
        201
    );
}

#[tokio::test]
async fn test_emails_create_with_optional_params() {
    let mut server = Server::new_async().await;
    let id = "eml-opt1-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/emails").as_str(),
        )
        .with_status(201)
        .with_body(
            json!({"data": {"id": id, "type": "emails", "attributes": {"status": "string"}}})
                .to_string(),
        )
        .create();
    let em = Emails::new(ORG_ID, TOKEN, server.url());
    let preset = PresetRelationship::new("eml-preset-1");
    let r = em
        .create(
            "https://s3.ex/file",
            "$sig",
            "lorem.pdf",
            false,
            Some(&EmailMetaData {
                sender_name: "Test Sender".into(),
                recipient_email: "recipient@example.com".into(),
                recipient_name: "Test Recipient".into(),
                reply_email: "reply@example.com".into(),
                reply_name: "Reply Name".into(),
                subject: "Test Subject".into(),
                content: "Test content".into(),
            }),
            Some(&preset),
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}

#[tokio::test]
async fn test_emails_create_with_relationships() {
    let mut server = Server::new_async().await;
    let id = "eml-rel1-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/emails").as_str(),
        )
        .with_status(201)
        .with_body(
            json!({"data": {"id": id, "type": "emails", "attributes": {"status": "string"}}})
                .to_string(),
        )
        .create();
    let em = Emails::new(ORG_ID, TOKEN, server.url());
    let preset = PresetRelationship::new("p1");
    let r = em
        .create(
            "https://s3/f",
            "$s",
            "f.pdf",
            false,
            Some(&EmailMetaData {
                sender_name: "x".into(),
                recipient_email: "r@example.com".into(),
                recipient_name: "R".into(),
                reply_email: "reply@example.com".into(),
                reply_name: "Reply".into(),
                subject: "Subj".into(),
                content: "Body".into(),
            }),
            Some(&preset),
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}
