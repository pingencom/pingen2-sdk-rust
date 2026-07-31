mod common;
use common::*;

use mockito::{Matcher, Server};
use pingen2_sdk::api::*;
use pingen2_sdk::error::PingenError;
use pingen2_sdk::{EbillMetaData, EbillRelationships, PresetRelationship};
use serde_json::json;

fn ebill_json(id: &str) -> String {
    json!({
        "data": { "id": id, "type": "ebills", "attributes": {
            "status": "string", "file_original_name": "test.pdf", "file_pages": 2,
            "recipient_identifier": "41100010014282213",
            "invoice_number": "Invoice 8051", "invoice_date": "2025-10-01",
            "invoice_due_date": "2025-10-30", "invoice_value": 1250.3,
            "invoice_currency": "CHF", "price_currency": "CHF", "price_value": 1.25,
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
async fn test_ebills_get_details() {
    let mut server = Server::new_async().await;
    let id = "ebill001-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/ebills/{id}").as_str(),
        )
        .with_status(200)
        .with_body(ebill_json(id))
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    let r = eb.get_details(id, None).await.unwrap();
    assert_eq!(r.status_code, 200);
    assert_eq!(r.id, id);
    assert_eq!(r.attributes.status.as_deref(), Some("string"));
    assert_eq!(r.attributes.invoice_number.as_deref(), Some("Invoice 8051"));
    assert_eq!(r.attributes.invoice_value, Some(1250.3));
    assert_eq!(r.attributes.price_currency.as_deref(), Some("CHF"));

    let rels: EbillRelationships = r.typed_relationships().unwrap();
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
async fn test_ebills_get_collection() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/ebills").as_str(),
        )
        .with_status(200)
        .with_body(
            json!({"data": [{"id": "ebill001", "type": "ebills", "attributes": {}}]}).to_string(),
        )
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    assert_eq!(eb.get_collection(None).await.unwrap().status_code, 200);
}

#[tokio::test]
async fn test_ebills_create() {
    let mut server = Server::new_async().await;
    let id = "ebill002-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/ebills").as_str(),
        )
        .with_status(201)
        .with_body(
            json!({"data": {"id": id, "type": "ebills", "attributes": {"status": "string"}}})
                .to_string(),
        )
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    let r = eb
        .create("https://s3.ex/file", "$sig", "test.pdf", false, None, None)
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}

#[tokio::test]
async fn test_ebills_upload_and_create() {
    let mut server = Server::new_async().await;
    let (_um, _sm) = stub_file_upload(&mut server);
    let id = "ebill003-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _pm = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/ebills").as_str(),
        )
        .with_status(201)
        .with_body(
            json!({"data": {"id": id, "type": "ebills", "attributes": {"status": "string"}}})
                .to_string(),
        )
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    assert_eq!(
        eb.upload_and_create(&fixture_pdf(), "test.pdf", false, None, None)
            .await
            .unwrap()
            .status_code,
        201
    );
}

#[tokio::test]
async fn test_ebills_create_with_optional_params() {
    let mut server = Server::new_async().await;
    let id = "ebl-opt1-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/ebills").as_str(),
        )
        .with_status(201)
        .with_body(
            json!({"data": {"id": id, "type": "ebills", "attributes": {"status": "string"}}})
                .to_string(),
        )
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    let preset = PresetRelationship::new("RCP-001");
    let r = eb
        .create(
            "https://s3.ex/file",
            "$sig",
            "test.pdf",
            false,
            Some(&EbillMetaData {
                invoice_number: "INV-001".into(),
                invoice_date: "2026-01-01".into(),
                invoice_due_date: "2026-02-01".into(),
                recipient_identifier: "RCP-001".into(),
            }),
            Some(&preset),
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}

#[tokio::test]
async fn test_ebills_send() {
    let mut server = Server::new_async().await;
    let id = "ebl-send-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/deliveries/ebills/{id}/send").as_str(),
        )
        .match_body(Matcher::Json(json!({"data": {"id": id, "type": "ebills"}})))
        .with_status(200)
        .with_body(ebill_json(id))
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    let r = eb.send(id).await.unwrap();
    assert_eq!(r.status_code, 200);
    assert_eq!(r.id, id);
    assert_eq!(r.attributes.status.as_deref(), Some("string"));
}

#[tokio::test]
async fn test_ebills_cancel() {
    let mut server = Server::new_async().await;
    let id = "ebl-cncl-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/deliveries/ebills/{id}/cancel").as_str(),
        )
        .with_status(202)
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    let r = eb.cancel(id).await.unwrap();
    assert_eq!(r.status_code, 202);
}

#[tokio::test]
async fn test_ebills_delete() {
    let mut server = Server::new_async().await;
    let id = "ebl-delx-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "DELETE",
            format!("/organisations/{ORG_ID}/deliveries/ebills/{id}").as_str(),
        )
        .with_status(204)
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    let r = eb.delete(id).await.unwrap();
    assert_eq!(r.status_code, 204);
}

#[tokio::test]
async fn test_ebills_delete_unauthorized() {
    let mut server = Server::new_async().await;
    let id = "ebl-delx-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "DELETE",
            format!("/organisations/{ORG_ID}/deliveries/ebills/{id}").as_str(),
        )
        .with_status(401)
        .with_body(access_denied_json())
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    let err = eb.delete(id).await;
    assert!(matches!(err, Err(PingenError::Api { status: 401, .. })));
}

#[tokio::test]
async fn test_ebills_get_file() {
    let mut server = Server::new_async().await;
    let id = "ebl-file-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let pdf_bytes: &[u8] = b"%PDF-1.4\r\n\xFF\xD8\xFE\x00binary-content";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/ebills/{id}/file").as_str(),
        )
        .with_status(200)
        .with_body(pdf_bytes)
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    let content = eb.get_file(id).await.unwrap();
    assert_eq!(content, pdf_bytes);
}

#[tokio::test]
async fn test_ebills_create_with_relationships() {
    let mut server = Server::new_async().await;
    let id = "ebl-rel1-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/ebills").as_str(),
        )
        .with_status(201)
        .with_body(
            json!({"data": {"id": id, "type": "ebills", "attributes": {"status": "string"}}})
                .to_string(),
        )
        .create();
    let eb = Ebills::new(ORG_ID, TOKEN, server.url());
    let preset = PresetRelationship::new("p1");
    let r = eb
        .create(
            "https://s3/f",
            "$s",
            "f.pdf",
            false,
            Some(&EbillMetaData {
                invoice_number: "INV-002".into(),
                invoice_date: "2026-01-15".into(),
                invoice_due_date: "2026-02-15".into(),
                recipient_identifier: "RCP-002".into(),
            }),
            Some(&preset),
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}
