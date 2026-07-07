mod common;
use common::*;

use pingen2_sdk::api::client::PingenClient;

#[test]
fn test_pingen_client_new() {
    let client = PingenClient::new("my_token");
    let _letters = client.letters(ORG_ID);
    let _batches = client.batches(ORG_ID);
    let _letter_events = client.letter_events(ORG_ID);
    let _batch_events = client.batch_events(ORG_ID);
    let _orgs = client.organisations();
    let _users = client.users();
    let _user_assoc = client.user_associations();
    let _webhooks = client.webhooks(ORG_ID);
    let _ebills = client.ebills(ORG_ID);
    let _emails = client.emails(ORG_ID);
}

#[test]
fn test_pingen_client_new_staging() {
    let client = PingenClient::new_staging("my_token");
    let _letters = client.letters(ORG_ID);
    let _batches = client.batches(ORG_ID);
    let _orgs = client.organisations();
    let _users = client.users();
    let _user_assoc = client.user_associations();
    let _webhooks = client.webhooks(ORG_ID);
    let _ebills = client.ebills(ORG_ID);
    let _emails = client.emails(ORG_ID);
}
