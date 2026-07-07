use pingen2_sdk::webhook::IncomingWebhook;
use pingen2_sdk::{
    WebhookChannelSubscriptionAttributes, WebhookEventAttributes, WebhookEventRelationships,
};

fn make_signature(payload: &str, secret: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

#[test]
fn test_webhook_event_type_issues() {
    let payload = r#"{"data":{"id":"ev1","type":"issues","attributes":{"reason":"Content failed","url":"https://hook","created_at":"2024-01-01"}}}"#;
    let secret = "test_secret";
    let sig = make_signature(payload, secret);
    let event = IncomingWebhook::construct_event(payload, &sig, secret).unwrap();

    assert_eq!(event.event_type(), Some("issues"));
    assert_eq!(event.body, payload);
}

#[test]
fn test_webhook_event_type_sent() {
    let payload = r#"{"data":{"id":"ev2","type":"sent","attributes":{"url":"https://hook","created_at":"2024-01-01"}}}"#;
    let secret = "s";
    let sig = make_signature(payload, secret);
    let event = IncomingWebhook::construct_event(payload, &sig, secret).unwrap();

    assert_eq!(event.event_type(), Some("sent"));
}

#[test]
fn test_webhook_event_as_resource() {
    let payload = r#"{"data":{"id":"ev3","type":"undeliverable","attributes":{"reason":"Bad address","corrected_address":{"name":"Hans","street":"Main St","number":"5","zip":"8000","city":"Zurich"},"url":"https://hook","created_at":"2024-01-01"}}}"#;
    let secret = "sec";
    let sig = make_signature(payload, secret);
    let event = IncomingWebhook::construct_event(payload, &sig, secret).unwrap();

    let resource = event.as_resource().unwrap();
    assert_eq!(resource.data.id, "ev3");
    assert_eq!(resource.data.resource_type, "undeliverable");

    let attrs: WebhookEventAttributes = resource.data.typed_attributes().unwrap();
    assert_eq!(attrs.reason.as_deref(), Some("Bad address"));
    let addr = attrs.corrected_address.unwrap();
    assert_eq!(addr.name.as_deref(), Some("Hans"));
    assert_eq!(addr.city.as_deref(), Some("Zurich"));
}

#[test]
fn test_webhook_event_with_relationships() {
    let payload = r#"{"data":{"id":"ev5","type":"issues","attributes":{"reason":"Failed"},"relationships":{"organisation":{"data":{"id":"org-1","type":"organisations"}},"letter":{"data":{"id":"let-1","type":"letters"}},"event":{"data":{"id":"evt-1","type":"letters_events"}}}}}"#;
    let secret = "s";
    let sig = make_signature(payload, secret);
    let event = IncomingWebhook::construct_event(payload, &sig, secret).unwrap();

    let resource = event.as_resource().unwrap();
    let rels: WebhookEventRelationships = resource.data.typed_relationships().unwrap();
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
        rels.letter.as_ref().unwrap().data.as_ref().unwrap().id,
        "let-1"
    );
    assert_eq!(
        rels.event.as_ref().unwrap().data.as_ref().unwrap().id,
        "evt-1"
    );
}

#[test]
fn test_webhook_event_channel_subscriptions() {
    let payload = r#"{"data":{"id":"ev4","type":"channel_subscriptions","attributes":{"identifier":"411","email":"test@test.com","name":"Test User","status":"requested","url":"https://hook","created_at":"2024-01-01"}}}"#;
    let secret = "key";
    let sig = make_signature(payload, secret);
    let event = IncomingWebhook::construct_event(payload, &sig, secret).unwrap();

    assert_eq!(event.event_type(), Some("channel_subscriptions"));
    let resource = event.as_resource().unwrap();
    let attrs: WebhookChannelSubscriptionAttributes = resource.data.typed_attributes().unwrap();
    assert_eq!(attrs.identifier.as_deref(), Some("411"));
    assert_eq!(attrs.email.as_deref(), Some("test@test.com"));
    assert_eq!(attrs.status.as_deref(), Some("requested"));
}

#[test]
fn test_webhook_event_type_none_for_invalid() {
    let payload = r#"{"not_data": "invalid"}"#;
    let secret = "k";
    let sig = make_signature(payload, secret);
    let event = IncomingWebhook::construct_event(payload, &sig, secret).unwrap();

    assert!(event.event_type().is_none());
    assert!(event.as_resource().is_none());
}
