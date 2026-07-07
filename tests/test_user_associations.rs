mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use serde_json::json;

#[tokio::test]
async fn test_user_associations_get_collection() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("GET", "/user/associations")
        .with_status(200)
        .with_body(
            json!({"data": [{"id": "assoc001", "type": "user_associations", "attributes": {}}]})
                .to_string(),
        )
        .create();
    let ua = UserAssociations::new(TOKEN, server.url());
    assert_eq!(ua.get_collection(None).await.unwrap().status_code, 200);
}
