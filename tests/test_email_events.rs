mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use pingen2_sdk::EventRelationships;
use serde_json::json;

#[tokio::test]
async fn test_email_events_get_collection() {
    let mut server = Server::new_async().await;
    let eid = "email001-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/emails/{eid}/events").as_str(),
        )
        .with_status(200)
        .with_body(
            json!({"data": [{"id": "ev001", "type": "deliverables_events", "attributes": {
                "code": "email_sent", "name": "Email sent", "producer": "Pingen",
                "location": "", "has_image": false,
                "emitted_at": "2021-11-19T09:42:48+0100"
            }, "relationships": {
                "email": {
                    "data": { "id": eid, "type": "emails" },
                    "links": { "related": format!("https://api/emails/{eid}") }
                }
            }}]})
            .to_string(),
        )
        .create();
    let ee = EmailEvents::new(ORG_ID, TOKEN, server.url());
    let r = ee.get_collection(eid, None).await.unwrap();
    assert_eq!(r.status_code, 200);
    assert_eq!(r.data.len(), 1);
    assert_eq!(r.data[0].attributes.code.as_deref(), Some("email_sent"));

    let rels: EventRelationships = r.data[0].typed_relationships().unwrap();
    assert_eq!(rels.email.as_ref().unwrap().data.as_ref().unwrap().id, eid);
}
