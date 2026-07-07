mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::requestor::ApiRequestor;
use pingen2_sdk::api::*;
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
async fn test_requestor_build_url_with_bad_base() {
    let req = ApiRequestor::new(TOKEN, "not-a-valid-url");
    let mut params = HashMap::new();
    params.insert("key".to_string(), "val".to_string());
    let result = req.get("/path", Some(&params)).await;
    assert!(result.is_err());
}
