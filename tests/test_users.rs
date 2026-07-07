mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use serde_json::json;

#[tokio::test]
async fn test_users_get_details() {
    let mut server = Server::new_async().await;
    let _m = server
        .mock("GET", "/user")
        .with_status(200)
        .with_body(
            json!({"data": {"id": "user001", "type": "users", "attributes": {}}}).to_string(),
        )
        .create();
    let users = Users::new(TOKEN, server.url());
    assert_eq!(users.get_details(None).await.unwrap().status_code, 200);
}
