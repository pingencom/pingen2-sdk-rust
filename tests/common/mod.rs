#![allow(dead_code)]

use mockito::Server;
use serde_json::json;
use std::path::PathBuf;

pub const ORG_ID: &str = "testxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
pub const TOKEN: &str = "test_access_token";

pub fn fixture_pdf() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/lorem.pdf")
}

pub fn letter_json(id: &str) -> String {
    json!({
        "data": { "id": id, "type": "letters", "attributes": {
            "status": "string", "file_original_name": "lorem.pdf", "file_pages": 2,
            "address": "Hans Meier\nExample street 4\n8000 Zürich\nSwitzerland",
            "address_position": "left", "country": "CH",
            "delivery_product": "fast", "print_mode": "simplex", "print_spectrum": "color",
            "price_currency": "CHF", "price_value": 1.25,
            "paper_types": ["normal", "qr"],
            "fonts": [{"name": "Helvetica", "is_embedded": true}],
            "source": "api", "tracking_number": "98.1234.11",
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
            },
            "batch": {
                "data": { "id": "batch-1", "type": "batches" },
                "links": { "related": "https://api/batches/batch-1" }
            }
        }}
    })
    .to_string()
}

pub fn batch_json(id: &str) -> String {
    json!({
        "data": { "id": id, "type": "batches", "attributes": {
            "name": "Test Batch", "icon": "campaign", "status": "string",
            "file_original_name": "lorem.pdf", "letter_count": 2,
            "address_position": "left", "print_mode": "simplex", "print_spectrum": "color",
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

pub fn webhook_json(id: &str) -> String {
    json!({
        "data": { "id": id, "type": "webhooks", "attributes": {
            "event_category": "issues", "url": "https://valid-url", "signing_key": "d09a"
        }}
    })
    .to_string()
}

pub fn access_denied_json() -> String {
    json!({"errors": {"code": "access_denied", "title": "Access denied."}}).to_string()
}

pub fn stub_file_upload(server: &mut Server) -> (mockito::Mock, mockito::Mock) {
    let s3_url = format!("{}/s3-bucket", server.url());
    let upload_mock = server
        .mock("GET", "/file-upload")
        .with_status(200)
        .with_header("content-type", "application/vnd.api+json")
        .with_body(
            json!({
                "data": { "id": "xx", "type": "file_uploads", "attributes": {
                    "url": s3_url, "url_signature": "$2y$sig", "expires_at": "2099-01-01"
                }}
            })
            .to_string(),
        )
        .create();
    let s3_mock = server.mock("PUT", "/s3-bucket").with_status(201).create();
    (upload_mock, s3_mock)
}
