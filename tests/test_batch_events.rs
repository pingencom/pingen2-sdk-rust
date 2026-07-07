mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use serde_json::json;

#[tokio::test]
async fn test_batch_events_get_collection() {
    let mut server = Server::new_async().await;
    let bid = "batch001-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/batches/{bid}/events").as_str(),
        )
        .with_status(200)
        .with_body(json!({"data": []}).to_string())
        .create();
    let be = BatchEvents::new(ORG_ID, TOKEN, server.url());
    assert_eq!(be.get_collection(bid, None).await.unwrap().status_code, 200);
}
