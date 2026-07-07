mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use pingen2_sdk::error::PingenError;
use pingen2_sdk::{
    AddressPosition, DeliveryProduct, LetterMetaData, LetterRecipient, LetterRelationships,
    LetterSender, PaperType, PresetRelationship, PrintMode, PrintSpectrum,
};
use serde_json::json;

#[tokio::test]
async fn test_letters_get_details() {
    let mut server = Server::new_async().await;
    let id = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/letters/{id}").as_str(),
        )
        .with_status(200)
        .with_header("content-type", "application/vnd.api+json")
        .with_header("x-request-id", "requestx-xxxx-xxxx-xxxx-xxxxxxxxxxx1")
        .with_body(letter_json(id))
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let r = letters.get_details(id, None).await.unwrap();

    assert_eq!(r.status_code, 200);
    assert_eq!(r.id, id);
    assert_eq!(r.request_id(), Some("requestx-xxxx-xxxx-xxxx-xxxxxxxxxxx1"));
    assert_eq!(r.attributes.status.as_deref(), Some("string"));
    assert_eq!(
        r.attributes.file_original_name.as_deref(),
        Some("lorem.pdf")
    );
    assert_eq!(r.attributes.file_pages, Some(2));
    assert_eq!(r.attributes.address_position.as_deref(), Some("left"));
    assert_eq!(r.attributes.country.as_deref(), Some("CH"));
    assert_eq!(r.attributes.delivery_product.as_deref(), Some("fast"));
    assert_eq!(r.attributes.price_currency.as_deref(), Some("CHF"));
    assert_eq!(r.attributes.price_value, Some(1.25));
    assert_eq!(r.attributes.paper_types.as_ref().unwrap().len(), 2);
    assert_eq!(r.attributes.fonts.as_ref().unwrap()[0].name, "Helvetica");
    assert!(r.attributes.fonts.as_ref().unwrap()[0].is_embedded);
    assert_eq!(r.attributes.source.as_deref(), Some("api"));
    assert_eq!(r.attributes.tracking_number.as_deref(), Some("98.1234.11"));

    let rels: LetterRelationships = r.typed_relationships().unwrap();
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
    assert_eq!(
        rels.batch.as_ref().unwrap().data.as_ref().unwrap().id,
        "batch-1"
    );
}

#[tokio::test]
async fn test_letters_get_collection() {
    let mut server = Server::new_async().await;
    let id = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/letters").as_str(),
        )
        .with_status(200)
        .with_header("content-type", "application/vnd.api+json")
        .with_header("x-request-id", "requestx-xxxx-xxxx-xxxx-xxxxxxxxxxx1")
        .with_body(
            json!({"data": [{"id": id, "type": "letters", "attributes": {"status": "valid"}, "relationships": {"organisation": {"data": {"id": "org-1", "type": "organisations"}}}}], "meta": {"total": 1}}).to_string(),
        )
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let r = letters.get_collection(None).await.unwrap();

    assert_eq!(r.status_code, 200);
    assert_eq!(r.data[0].id, id);
    assert_eq!(r.data[0].attributes.status.as_deref(), Some("valid"));
    assert_eq!(r.meta.as_ref().unwrap().total, Some(1));

    let rels: LetterRelationships = r.data[0].typed_relationships().unwrap();
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
async fn test_letters_create() {
    let mut server = Server::new_async().await;
    let id = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxx11";
    let _m = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/letters").as_str(),
        )
        .with_status(201)
        .with_header("content-type", "application/vnd.api+json")
        .with_header("x-request-id", "requestx-xxxx-xxxx-xxxx-xxxxxxxxxxx2")
        .with_body(letter_json(id))
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let r = letters
        .create(
            "https://s3.ex/file",
            "$sig",
            "lorem.pdf",
            AddressPosition::Left,
            false,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(r.status_code, 201);
    assert_eq!(r.id, id);
}

#[tokio::test]
async fn test_letters_upload_and_create() {
    let mut server = Server::new_async().await;
    let (_um, _sm) = stub_file_upload(&mut server);
    let id = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxx111";
    let _pm = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/letters").as_str(),
        )
        .with_status(201)
        .with_header("content-type", "application/vnd.api+json")
        .with_header("x-request-id", "requestx-xxxx-xxxx-xxxx-xxxxxxxxxxx3")
        .with_body(letter_json(id))
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let r = letters
        .upload_and_create(
            &fixture_pdf(),
            "lorem.pdf",
            AddressPosition::Left,
            false,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(r.status_code, 201);
    assert_eq!(r.id, id);
}

#[tokio::test]
async fn test_letters_send() {
    let mut server = Server::new_async().await;
    let id = "testsend-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/deliveries/letters/{id}/send").as_str(),
        )
        .with_status(200)
        .with_header("x-request-id", "requestx-xxxx-xxxx-xxxx-xxxxxxxxx332")
        .with_body(letter_json(id))
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let r = letters
        .send(
            id,
            DeliveryProduct::Fast,
            PrintMode::Simplex,
            PrintSpectrum::Color,
        )
        .await
        .unwrap();

    assert_eq!(r.status_code, 200);
    assert_eq!(r.request_id(), Some("requestx-xxxx-xxxx-xxxx-xxxxxxxxx332"));
}

#[tokio::test]
async fn test_letters_cancel() {
    let mut server = Server::new_async().await;
    let id = "testsend-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/deliveries/letters/{id}/cancel").as_str(),
        )
        .with_status(202)
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let r = letters.cancel(id).await.unwrap();
    assert_eq!(r.status_code, 202);
}

#[tokio::test]
async fn test_letters_delete() {
    let mut server = Server::new_async().await;
    let id = "testdelx-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock(
            "DELETE",
            format!("/organisations/{ORG_ID}/deliveries/letters/{id}").as_str(),
        )
        .with_status(204)
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let r = letters.delete(id).await.unwrap();
    assert_eq!(r.status_code, 204);
}

#[tokio::test]
async fn test_letters_delete_unauthorized() {
    let mut server = Server::new_async().await;
    let id = "testdelx-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock(
            "DELETE",
            format!("/organisations/{ORG_ID}/deliveries/letters/{id}").as_str(),
        )
        .with_status(401)
        .with_body(access_denied_json())
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let err = letters.delete(id).await;
    assert!(matches!(err, Err(PingenError::Api { status: 401, .. })));
}

#[tokio::test]
async fn test_letters_edit() {
    let mut server = Server::new_async().await;
    let id = "testedit-xxxx-xxxx-xxxx-xxxxxxxxx551";
    let _m = server
        .mock(
            "PATCH",
            format!("/organisations/{ORG_ID}/deliveries/letters/{id}").as_str(),
        )
        .with_status(200)
        .with_header("x-request-id", "req-edit")
        .with_body(letter_json(id))
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let r = letters
        .edit(id, &[PaperType::Normal, PaperType::Qr])
        .await
        .unwrap();
    assert_eq!(r.status_code, 200);
}

#[tokio::test]
async fn test_letters_calculate_price() {
    let mut server = Server::new_async().await;
    let _m = server.mock("POST", format!("/organisations/{ORG_ID}/deliveries/letters/price-calculator").as_str())
        .with_status(200)
        .with_header("content-type", "application/vnd.api+json")
        .with_header("x-request-id", "requestx-xxxx-xxxx-xxxx-xxxxxxxx1332")
        .with_body(json!({"data": {"id": "xx", "type": "letter_price_calculator", "attributes": {"currency": "CHF", "price": 12.12}}}).to_string())
        .create();

    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let r = letters
        .calculate_price(
            "CH",
            &[PaperType::Normal, PaperType::Qr],
            PrintMode::Simplex,
            PrintSpectrum::Color,
            DeliveryProduct::Fast,
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 200);
    assert_eq!(r.attributes.currency.as_deref(), Some("CHF"));
}

#[tokio::test]
async fn test_letters_get_file() {
    let mut server = Server::new_async().await;
    let id = "orig-fil-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let pdf_bytes: &[u8] = b"%PDF-1.4\r\n\xFF\xD8\xFE\x00binary-content";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/letters/{id}/file").as_str(),
        )
        .with_status(200)
        .with_body(pdf_bytes)
        .create();
    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let content = letters.get_file(id).await.unwrap();
    assert_eq!(content, pdf_bytes);
}

#[tokio::test]
async fn test_letters_create_with_optional_params() {
    let mut server = Server::new_async().await;
    let id = "let-opt1-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/letters").as_str(),
        )
        .with_status(201)
        .with_body(letter_json(id))
        .create();
    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let preset = PresetRelationship::new("p1");
    let r = letters
        .create(
            "https://s3.ex/file",
            "$sig",
            "lorem.pdf",
            AddressPosition::Left,
            true,
            Some(DeliveryProduct::Fast),
            Some(PrintMode::Simplex),
            Some(PrintSpectrum::Color),
            Some(&LetterMetaData {
                recipient: LetterRecipient {
                    name: String::new(),
                    street: None,
                    pobox: None,
                    number: None,
                    zip: String::new(),
                    city: String::new(),
                    country: String::new(),
                },
                sender: LetterSender {
                    name: String::new(),
                    street: None,
                    pobox: None,
                    number: None,
                    zip: String::new(),
                    city: String::new(),
                    country: String::new(),
                },
            }),
            Some(&preset),
        )
        .await
        .unwrap();
    assert_eq!(r.status_code, 201);
}

#[tokio::test]
async fn test_letters_create_with_relationships() {
    let mut server = Server::new_async().await;
    let id = "let-rel1-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "POST",
            format!("/organisations/{ORG_ID}/deliveries/letters").as_str(),
        )
        .with_status(201)
        .with_body(letter_json(id))
        .create();
    let letters = Letters::new(ORG_ID, TOKEN, server.url());
    let preset = PresetRelationship::new("p1");
    let r = letters
        .create(
            "https://s3/f",
            "$s",
            "f.pdf",
            AddressPosition::Left,
            false,
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

#[tokio::test]
async fn test_letters_create_auto_send_requires_options() {
    let letters = Letters::new(ORG_ID, TOKEN, "http://unused");
    let result = letters
        .create(
            "https://s3/f",
            "$s",
            "f.pdf",
            AddressPosition::Left,
            true,
            None,
            None,
            None,
            None,
            None,
        )
        .await;
    let err = result.unwrap_err();
    assert!(matches!(err, pingen2_sdk::PingenError::Validation(_)));
    assert!(err.to_string().contains("auto_send is true"));
}
