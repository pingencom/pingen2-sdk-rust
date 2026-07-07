mod common;
use common::*;

use mockito::Server;
use pingen2_sdk::api::*;
use pingen2_sdk::OrganisationRelationships;
use serde_json::json;

fn organisation_json(id: &str) -> String {
    json!({
        "data": { "id": id, "type": "organisations", "attributes": {
            "name": "ACME GmbH", "status": "active", "plan": "free",
            "billing_mode": "prepaid", "billing_currency": "CHF",
            "billing_balance": 11.23, "missing_credits": 0,
            "edition": "string", "default_country": "CH",
            "default_address_position": "left",
            "data_retention_addresses": 18, "data_retention_pdf": 12,
            "limits_monthly_letters_count": 5000,
            "color": "#0758FF", "flags": ["string"],
            "created_at": "2020-11-19T09:42:48+0100",
            "updated_at": "2020-11-19T09:42:48+0100"
        }, "relationships": {
            "associations": {
                "links": { "related": { "href": "https://api/associations", "meta": { "count": 0 }}}
            }
        }}
    })
    .to_string()
}

#[tokio::test]
async fn test_organisations_get_details() {
    let mut server = Server::new_async().await;
    let id = "orgxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxx1";
    let _m = server
        .mock("GET", format!("/organisations/{id}").as_str())
        .with_status(200)
        .with_header("content-type", "application/vnd.api+json")
        .with_body(organisation_json(id))
        .create();
    let orgs = Organisations::new(TOKEN, server.url());
    let r = orgs.get_details(id, None).await.unwrap();
    assert_eq!(r.status_code, 200);
    assert_eq!(r.id, id);
    assert_eq!(r.resource_type, "organisations");
    assert_eq!(r.attributes.name.as_deref(), Some("ACME GmbH"));
    assert_eq!(r.attributes.status.as_deref(), Some("active"));
    assert_eq!(r.attributes.plan.as_deref(), Some("free"));
    assert_eq!(r.attributes.billing_mode.as_deref(), Some("prepaid"));
    assert_eq!(r.attributes.billing_currency.as_deref(), Some("CHF"));
    assert_eq!(r.attributes.billing_balance, Some(11.23));
    assert_eq!(r.attributes.default_country.as_deref(), Some("CH"));
    assert_eq!(
        r.attributes.default_address_position.as_deref(),
        Some("left")
    );
    assert_eq!(r.attributes.data_retention_addresses, Some(18));
    assert_eq!(r.attributes.data_retention_pdf, Some(12));
    assert_eq!(r.attributes.limits_monthly_letters_count, Some(5000));
    assert_eq!(r.attributes.color.as_deref(), Some("#0758FF"));

    let rels: OrganisationRelationships = r.typed_relationships().unwrap();
    assert!(rels.associations.is_some());
}

#[tokio::test]
async fn test_organisations_get_collection() {
    let mut server = Server::new_async().await;
    let id = "orgxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxx11";
    let _m = server
        .mock("GET", "/organisations")
        .with_status(200)
        .with_body(
            json!({"data": [{"id": id, "type": "organisations", "attributes": {
                "name": "ACME GmbH", "status": "active", "plan": "free",
                "billing_mode": "prepaid", "billing_currency": "CHF",
                "billing_balance": 11.23, "color": "#0758FF"
            }}]})
            .to_string(),
        )
        .create();
    let orgs = Organisations::new(TOKEN, server.url());
    let r = orgs.get_collection(None).await.unwrap();
    assert_eq!(r.status_code, 200);
    assert_eq!(r.data[0].attributes.name.as_deref(), Some("ACME GmbH"));
    assert_eq!(
        r.data[0].attributes.billing_currency.as_deref(),
        Some("CHF")
    );
}
