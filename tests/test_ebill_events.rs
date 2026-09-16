mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use pingen2_sdk::EventRelationships;
use serde_json::json;

#[tokio::test]
async fn test_ebill_events_get_collection() {
    let mut server = Server::new_async().await;
    let eid = "ebill001-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
    let _m = server
        .mock(
            "GET",
            format!("/organisations/{ORG_ID}/deliveries/ebills/{eid}/events").as_str(),
        )
        .with_status(200)
        .with_body(
            json!({"data": [{"id": "ev001", "type": "deliverables_events", "attributes": {
                "code": "ebill_sent", "name": "Sent", "producer": "Pingen",
                "emitted_at": "2020-11-19T09:42:48+0100",
                "created_at": "2020-11-19T09:42:48+0100",
                "updated_at": "2020-11-19T09:42:48+0100"
            }, "relationships": {
                "ebill": {
                    "data": { "id": eid, "type": "ebills" },
                    "links": { "related": format!("https://api/ebills/{eid}") }
                }
            }}]})
            .to_string(),
        )
        .create();
    let ee = EbillEvents::new(ORG_ID, TOKEN, server.url());
    let r = ee.get_collection(eid, None).await.unwrap();
    assert_eq!(r.status_code, 200);
    assert_eq!(r.data[0].id, "ev001");
    assert_eq!(r.data[0].resource_type, "deliverables_events");
    assert_eq!(r.data[0].attributes.code.as_deref(), Some("ebill_sent"));
    assert_eq!(r.data[0].attributes.name.as_deref(), Some("Sent"));
    assert_eq!(r.data[0].attributes.producer.as_deref(), Some("Pingen"));

    let rels: EventRelationships = r.data[0].typed_relationships().unwrap();
    assert_eq!(rels.ebill.as_ref().unwrap().data.as_ref().unwrap().id, eid);
    assert!(rels.letter.is_none());
}
