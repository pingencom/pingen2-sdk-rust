mod common;
use common::*;

use mockito::{Matcher, Server};
use pingen2_sdk::api::*;
use pingen2_sdk::error::PingenError;
use pingen2_sdk::{
    AddressPosition, BatchIcon, ChannelType, DeliveryProduct, GroupingType, PaperType,
    PresetRelationship, PrintMode, PrintSpectrum, SplitPosition, SplitType,
};
use serde_json::json;

#[tokio::test]
async fn test_batches_get_details() {
    let mut server = Server::new_async().await;
    let id = "orig-bat-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/batches/{id}").as_str(),
        )
        .with_status(200)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(
        batches.get_details(id, None).await.unwrap().status_code,
        200
    );
}

#[tokio::test]
async fn test_batches_get_details_deserializes_new_attributes() {
    let mut server = Server::new_async().await;
    let id = "orig-bat-xxxx-xxxx-xxxx-xxxxxxxxxxx9";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/batches/{id}").as_str(),
        )
        .with_status(200)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    let r = batches.get_details(id, None).await.unwrap();
    assert_eq!(r.attributes.deliverable_count, Some(2));
    assert_eq!(r.attributes.channel_type.as_deref(), Some("post"));
}

#[tokio::test]
async fn test_batches_get_collection() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("GET", format!("/organisations/{ORG_ID}/batches").as_str())
        .with_status(200)
        .with_body(json!({"data": []}).to_string())
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(batches.get_collection(None).await.unwrap().status_code, 200);
}

#[tokio::test]
async fn test_batches_create() {
    let mut server = Server::new_async().await;
    let id = "batch002-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock("POST", format!("/organisations/{ORG_ID}/batches").as_str())
        // channel_type defaults to "post" on the wire when the caller passes None.
        .match_body(Matcher::PartialJson(json!({
            "data": { "type": "batches", "attributes": { "channel_type": "post" }}
        })))
        .with_status(201)
        .with_header("x-request-id", "req2")
        .with_body(batch_json(id))
        .create();

    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    let r = batches
        .create(
            "https://s3.ex/file",
            "$sig",
            "Test",
            BatchIcon::Campaign,
            "test.pdf",
            AddressPosition::Left,
            GroupingType::Zip,
            SplitType::Page,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}

#[tokio::test]
async fn test_batches_create_with_optional_params() {
    let mut server = Server::new_async().await;
    let id = "orig-bat-xxxx-xxxx-xxxx-xxxxxxxxxxx2";
    let _m = server
        .mock("POST", format!("/organisations/{ORG_ID}/batches").as_str())
        .with_status(201)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    let r = batches
        .create(
            "https://s3.ex/file",
            "$sig",
            "Test",
            BatchIcon::Campaign,
            "test.pdf",
            AddressPosition::Left,
            GroupingType::Zip,
            SplitType::Page,
            Some(10),
            Some(";"),
            Some(SplitPosition::FirstPage),
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}

#[tokio::test]
async fn test_batches_create_with_channel_type() {
    let mut server = Server::new_async().await;
    let id = "bat-chan-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock("POST", format!("/organisations/{ORG_ID}/batches").as_str())
        .match_body(Matcher::PartialJson(json!({
            "data": { "type": "batches", "attributes": { "channel_type": "email" }}
        })))
        .with_status(201)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    let r = batches
        .create(
            "https://s3.ex/file",
            "$sig",
            "Test",
            BatchIcon::Campaign,
            "test.pdf",
            AddressPosition::Left,
            GroupingType::Zip,
            SplitType::Page,
            None,
            None,
            None,
            Some(ChannelType::Email),
            None,
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}

#[tokio::test]
async fn test_batches_upload_and_create() {
    let mut server = Server::new_async().await;
    let (_um, _sm) = stub_file_upload(&mut server);
    let id = "orig-bat-xxxx-xxxx-xxxx-xxxxxxxxxxx3";
    let _pm = server
        .mock("POST", format!("/organisations/{ORG_ID}/batches").as_str())
        .with_status(201)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    let r = batches
        .upload_and_create(
            &fixture_pdf(),
            "Test",
            BatchIcon::Campaign,
            "test.pdf",
            AddressPosition::Left,
            GroupingType::Zip,
            SplitType::Page,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}

#[tokio::test]
async fn test_batches_send_post() {
    let mut server = Server::new_async().await;
    let id = "orig-bat-xxxx-xxxx-xxxx-xxxxxxxxxxx4";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/batches/{id}/send").as_str(),
        )
        .match_body(Matcher::PartialJson(json!({
            "data": {
                "id": id,
                "type": "batches_channel_post_advanced_send",
                "attributes": {
                    "delivery_products": [
                        { "country": "CH", "delivery_product": "fast" },
                        { "country": "DE", "delivery_product": "cheap" }
                    ],
                    "print_mode": "simplex",
                    "print_spectrum": "color"
                }
            }
        })))
        .with_status(200)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    let delivery_products = [
        pingen2_sdk::BatchDeliveryProduct::new("CH", DeliveryProduct::Fast),
        pingen2_sdk::BatchDeliveryProduct::new("DE", DeliveryProduct::Cheap),
    ];
    assert_eq!(
        batches
            .send_post(
                id,
                &delivery_products,
                PrintMode::Simplex,
                PrintSpectrum::Color
            )
            .await
            .unwrap()
            .status_code,
        200
    );
}

#[tokio::test]
async fn test_batches_send_email() {
    let mut server = Server::new_async().await;
    let id = "bat-mail-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/batches/{id}/send").as_str(),
        )
        .match_body(Matcher::PartialJson(json!({
            "data": {
                "id": id,
                "type": "batches_channel_email_send",
                "attributes": { "delivery_product": "electronic_email" }
            }
        })))
        .with_status(200)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(batches.send_email(id).await.unwrap().status_code, 200);
}

#[tokio::test]
async fn test_batches_send_ebill() {
    let mut server = Server::new_async().await;
    let id = "bat-ebil-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/batches/{id}/send").as_str(),
        )
        .match_body(Matcher::PartialJson(json!({
            "data": {
                "id": id,
                "type": "batches_channel_ebill_send",
                "attributes": { "delivery_product": "electronic_ebill" }
            }
        })))
        .with_status(200)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(batches.send_ebill(id).await.unwrap().status_code, 200);
}

#[tokio::test]
async fn test_batches_update() {
    let mut server = Server::new_async().await;
    let id = "bat-upd1-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/batches/{id}").as_str(),
        )
        .match_body(Matcher::PartialJson(json!({
            "data": {
                "id": id,
                "type": "batches",
                "attributes": { "name": "Updated Integration Batch", "icon": "rocket" }
            }
        })))
        .with_status(200)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(
        batches
            .update(
                id,
                Some("Updated Integration Batch"),
                Some(BatchIcon::Rocket)
            )
            .await
            .unwrap()
            .status_code,
        200
    );
}

#[tokio::test]
async fn test_batches_cancel() {
    let mut server = Server::new_async().await;
    let id = "orig-bat-xxxx-xxxx-xxxx-xxxxxxxxxxx5";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/batches/{id}/cancel").as_str(),
        )
        .with_status(202)
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(batches.cancel(id).await.unwrap().status_code, 202);
}

#[tokio::test]
async fn test_batches_delete_with_deliverables() {
    let mut server = Server::new_async().await;
    let id = "orig-bat-xxxx-xxxx-xxxx-xxxxxxxxxxx6";
    let _m = server
        .mock(
            "DELETE",
            format!("/organisations/{ORG_ID}/batches/{id}").as_str(),
        )
        .match_body(Matcher::PartialJson(json!({
            "data": {
                "id": id,
                "type": "batches",
                "attributes": { "with_letters": true, "with_deliverables": true }
            }
        })))
        .with_status(204)
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(batches.delete(id, true).await.unwrap().status_code, 204);
}

#[tokio::test]
async fn test_batches_delete_without_deliverables() {
    let mut server = Server::new_async().await;
    let id = "bat-del2-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "DELETE",
            format!("/organisations/{ORG_ID}/batches/{id}").as_str(),
        )
        .match_body(Matcher::PartialJson(json!({
            "data": {
                "id": id,
                "type": "batches",
                "attributes": { "with_letters": false, "with_deliverables": false }
            }
        })))
        .with_status(204)
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(batches.delete(id, false).await.unwrap().status_code, 204);
}

#[tokio::test]
async fn test_batches_delete_unauthorized() {
    let mut server = Server::new_async().await;
    let id = "batchdel-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "DELETE",
            format!("/organisations/{ORG_ID}/batches/{id}").as_str(),
        )
        .with_status(401)
        .with_body(access_denied_json())
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    let err = batches.delete(id, true).await;
    assert!(matches!(err, Err(PingenError::Api { status: 401, .. })));
}

#[tokio::test]
async fn test_batches_edit() {
    let mut server = Server::new_async().await;
    let id = "orig-bat-xxxx-xxxx-xxxx-xxxxxxxxxxx7";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/batches/{id}").as_str(),
        )
        .with_status(200)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(
        batches
            .edit(id, &[PaperType::Normal])
            .await
            .unwrap()
            .status_code,
        200
    );
}

#[tokio::test]
async fn test_batches_get_statistics() {
    let mut server = Server::new_async().await;
    let id = "orig-bat-xxxx-xxxx-xxxx-xxxxxxxxxxx8";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/batches/{id}/statistics").as_str(),
        )
        .with_status(200)
        .with_body(
            json!({"data": {"id": id, "type": "batch_statistics", "attributes": {}}}).to_string(),
        )
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    assert_eq!(batches.get_statistics(id).await.unwrap().status_code, 200);
}

#[tokio::test]
async fn test_batches_create_with_relationships() {
    let mut server = Server::new_async().await;
    let id = "bat-rel1-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock("POST", format!("/organisations/{ORG_ID}/batches").as_str())
        .with_status(201)
        .with_body(batch_json(id))
        .create();
    let batches = Batches::new(ORG_ID, TOKEN, server.url());
    let preset = PresetRelationship::new("p1");
    let r = batches
        .create(
            "https://s3/f",
            "$s",
            "b",
            BatchIcon::Information,
            "f.pdf",
            AddressPosition::Left,
            GroupingType::Zip,
            SplitType::Page,
            None,
            None,
            None,
            None,
            Some(&preset),
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}
