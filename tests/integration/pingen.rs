//! Integration tests for the Pingen Rust SDK against the staging environment.
//!
//! These tests make real HTTP calls and require valid staging credentials in
//! `.env` (see `tests/integration/common.rs`). They are `#[ignore]`d so the
//! normal unit-test run and CI skip them. Run them explicitly with:
//!
//! ```text
//! cargo test --test integration_pingen --test integration_oauth -- --ignored --test-threads=1
//! ```
//!
//! The suite walks every resource end to end: organisations, letters (happy /
//! cancel / delete), batches, webhooks, emails, e-bills and the user endpoints.
//! Cancel / delete / update steps rely on the `simulate_cancellable` document
//! and the sleeps to reach the required state and assert strictly. Each test
//! runs its steps sequentially and shares created resource IDs via local
//! variables.

mod common;

use pingen2_sdk::{
    AddressPosition, ApiResource, BatchIcon, DeliveryProduct, EbillAttributes, Ebills,
    GroupingType, PaperType, PingenError, PrintMode, PrintSpectrum, SplitType,
    WebhookEventCategory,
};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

fn params(entries: &[(&str, &str)]) -> HashMap<String, String> {
    entries
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

/// Creates an e-bill, returning `None` (after printing a `SKIPPED` message)
/// when the e-bill channel is not configured for the organisation.
async fn create_ebill(
    ebills: &Ebills,
    path: &Path,
    auto_send: bool,
) -> Option<ApiResource<EbillAttributes>> {
    let meta_data = common::build_ebill_meta_data();
    match ebills
        .upload_and_create(
            path,
            common::document_name(path),
            auto_send,
            Some(&meta_data),
            None,
        )
        .await
    {
        Ok(response) => Some(response),
        Err(PingenError::Api { body, .. }) if body.contains("conflict_missing_configuration") => {
            eprintln!("SKIPPED: E-Bill channel not configured.");
            None
        }
        Err(error) => panic!("creating the e-bill must succeed: {}", error),
    }
}

// =============================================================================
// Organisations
// =============================================================================

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn organisations_crud() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let organisations = client.organisations();

    let response = organisations
        .get_collection(None)
        .await
        .expect("listing organisations must succeed");
    assert_eq!(response.status_code, 200);
    assert!(!response.data.is_empty());
    assert!(response.data.iter().all(|item| !item.id.is_empty()));

    let paginated = organisations
        .get_collection(Some(&params(&[
            ("page[number]", "1"),
            ("page[limit]", "5"),
        ])))
        .await
        .expect("listing organisations paginated must succeed");
    assert_eq!(paginated.status_code, 200);

    let org_id = common::org_id(&client, &credentials).await;
    let detail = organisations
        .get_details(&org_id, None)
        .await
        .expect("fetching the organisation must succeed");
    assert_eq!(detail.status_code, 200);
    assert_eq!(detail.id, org_id);
}

// =============================================================================
// Letters
// =============================================================================

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn letters_happy_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let letters = client.letters(&org_id);
    let letter_events = client.letter_events(&org_id);

    let response = letters
        .get_collection(None)
        .await
        .expect("listing letters must succeed");
    assert_eq!(response.status_code, 200);

    let paginated = letters
        .get_collection(Some(&params(&[
            ("page[number]", "1"),
            ("page[limit]", "3"),
        ])))
        .await
        .expect("listing letters paginated must succeed");
    assert_eq!(paginated.status_code, 200);

    let path = common::document_path(common::FILE_NAME);
    let created = letters
        .upload_and_create(
            &path,
            common::document_name(&path),
            AddressPosition::Left,
            true,
            Some(DeliveryProduct::Cheap),
            Some(PrintMode::Simplex),
            Some(PrintSpectrum::Grayscale),
            None,
            None,
        )
        .await
        .expect("creating the letter must succeed");
    assert!(!created.id.is_empty());
    assert_eq!(created.attributes.status.as_deref(), Some("validating"));
    let letter_id = created.id;

    let detail = letters
        .get_details(&letter_id, None)
        .await
        .expect("fetching the letter must succeed");
    assert_eq!(detail.id, letter_id);

    println!("sleep 30 seconds so the collection contains the newly created letter");
    sleep(Duration::from_secs(30)).await;

    // Newest first, so the letter we just created must be on the first page.
    let collection = letters
        .get_collection(Some(&params(&[
            ("sort", "-created_at"),
            ("page[number]", "1"),
            ("page[limit]", "20"),
        ])))
        .await
        .expect("listing letters sorted must succeed");
    assert!(collection.data.iter().any(|item| item.id == letter_id));

    let detail = letters
        .get_details(&letter_id, None)
        .await
        .expect("fetching the letter again must succeed");
    assert_eq!(detail.id, letter_id);
    println!("Letter status: {:?}", detail.attributes.status);

    let events = letter_events
        .get_collection(&letter_id, None)
        .await
        .expect("listing letter events must succeed");
    assert_eq!(events.status_code, 200);
    println!("Letter events: {}", events.data.len());

    let file_content = letters
        .get_file(&letter_id)
        .await
        .expect("downloading the letter file must succeed");
    assert!(!file_content.is_empty());

    let price = letters
        .calculate_price(
            "CH",
            &[PaperType::Normal, PaperType::Normal],
            PrintMode::Simplex,
            PrintSpectrum::Grayscale,
            DeliveryProduct::Cheap,
        )
        .await
        .expect("calculating the letter price must succeed");
    assert!(!price.id.is_empty());
    println!(
        "Price calculator: currency={:?}, price={:?}",
        price.attributes.currency, price.attributes.price
    );

    let sent = letter_events
        .get_sent_collection(None)
        .await
        .expect("listing sent letter events must succeed");
    assert_eq!(sent.status_code, 200);

    let delivered = letter_events
        .get_delivered_collection(None)
        .await
        .expect("listing delivered letter events must succeed");
    assert_eq!(delivered.status_code, 200);

    let issues = letter_events
        .get_issue_collection(None)
        .await
        .expect("listing letter issue events must succeed");
    assert_eq!(issues.status_code, 200);

    let undeliverable = letter_events
        .get_undeliverable_collection(None)
        .await
        .expect("listing undeliverable letter events must succeed");
    assert_eq!(undeliverable.status_code, 200);
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn letters_cancel_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let letters = client.letters(&org_id);

    let path = common::document_path(common::FILE_NAME_CANCELLABLE);
    let created = letters
        .upload_and_create(
            &path,
            common::document_name(&path),
            AddressPosition::Left,
            true,
            Some(DeliveryProduct::Cheap),
            Some(PrintMode::Simplex),
            Some(PrintSpectrum::Grayscale),
            None,
            None,
        )
        .await
        .expect("creating the letter must succeed");
    assert!(!created.id.is_empty());
    assert_eq!(created.attributes.status.as_deref(), Some("validating"));
    let letter_id = created.id;

    println!("sleep 10 seconds so the letter reaches a cancellable state");
    sleep(Duration::from_secs(10)).await;

    let response = letters
        .cancel(&letter_id)
        .await
        .expect("cancelling the letter must succeed");
    assert_eq!(response.status_code, 202);
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn letters_delete_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let letters = client.letters(&org_id);

    let path = common::document_path(common::FILE_NAME);
    let created = letters
        .upload_and_create(
            &path,
            common::document_name(&path),
            AddressPosition::Right,
            false,
            Some(DeliveryProduct::Cheap),
            Some(PrintMode::Simplex),
            Some(PrintSpectrum::Grayscale),
            None,
            None,
        )
        .await
        .expect("creating the letter must succeed");
    assert!(!created.id.is_empty());
    assert_eq!(created.attributes.status.as_deref(), Some("validating"));
    let letter_id = created.id;

    println!("sleep 5 seconds so the letter reaches a deletable state");
    sleep(Duration::from_secs(5)).await;

    let response = letters
        .delete(&letter_id)
        .await
        .expect("deleting the letter must succeed");
    assert_eq!(response.status_code, 204);
    println!("Deleted letter: {}", letter_id);
}

// =============================================================================
// Batches
// =============================================================================

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn batches_happy_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let batches = client.batches(&org_id);
    let batch_events = client.batch_events(&org_id);

    let response = batches
        .get_collection(None)
        .await
        .expect("listing batches must succeed");
    assert_eq!(response.status_code, 200);

    let paginated = batches
        .get_collection(Some(&params(&[
            ("page[number]", "1"),
            ("page[limit]", "3"),
        ])))
        .await
        .expect("listing batches paginated must succeed");
    assert_eq!(paginated.status_code, 200);

    let path = common::document_path(common::FILE_NAME);
    let created = batches
        .upload_and_create(
            &path,
            "Integration Test Batch",
            BatchIcon::Document,
            common::document_name(&path),
            AddressPosition::Left,
            GroupingType::Merge,
            SplitType::QrInvoice,
            Some(2),
            None,
            None,
            None,
            None,
        )
        .await
        .expect("creating the batch must succeed");
    assert!(!created.id.is_empty());
    let batch_id = created.id;
    println!(
        "Created batch: {} (status: {:?})",
        batch_id, created.attributes.status
    );

    let detail = batches
        .get_details(&batch_id, None)
        .await
        .expect("fetching the batch must succeed");
    assert_eq!(detail.id, batch_id);
    println!("Batch status: {:?}", detail.attributes.status);

    println!("sleep 10 seconds so the batch reaches an updatable state");
    sleep(Duration::from_secs(10)).await;

    let updated = batches
        .update(
            &batch_id,
            Some("Updated Integration Batch"),
            Some(BatchIcon::Rocket),
        )
        .await
        .expect("updating the batch must succeed");
    assert!(
        matches!(updated.status_code, 200 | 202),
        "unexpected update status: {}",
        updated.status_code
    );

    let events = batch_events
        .get_collection(&batch_id, None)
        .await
        .expect("listing batch events must succeed");
    assert_eq!(events.status_code, 200);
    println!("Batch events: {}", events.data.len());

    let statistics = batches
        .get_statistics(&batch_id)
        .await
        .expect("fetching the batch statistics must succeed");
    assert_eq!(statistics.status_code, 200);
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn batches_delete_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let batches = client.batches(&org_id);

    let path = common::document_path(common::FILE_NAME_CANCELLABLE);
    let created = batches
        .upload_and_create(
            &path,
            "Integration Test Batch",
            BatchIcon::Document,
            common::document_name(&path),
            AddressPosition::Left,
            GroupingType::Merge,
            SplitType::QrInvoice,
            Some(2),
            None,
            None,
            None,
            None,
        )
        .await
        .expect("creating the batch must succeed");
    assert!(!created.id.is_empty());
    let batch_id = created.id;
    println!("Created batch: {}", batch_id);

    println!("sleep 10 seconds so the batch reaches a deletable state");
    sleep(Duration::from_secs(10)).await;

    let response = batches
        .delete(&batch_id, true)
        .await
        .expect("deleting the batch must succeed");
    assert_eq!(response.status_code, 204);
    println!("Deleted batch: {}", batch_id);
}

// =============================================================================
// Webhooks
// =============================================================================

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn webhooks_crud() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let webhooks = client.webhooks(&org_id);

    let response = webhooks
        .get_collection(None)
        .await
        .expect("listing webhooks must succeed");
    assert_eq!(response.status_code, 200);

    let created = webhooks
        .create(
            WebhookEventCategory::Issues,
            "https://httpbin.org/post",
            "integration-test-signing-key-32c",
        )
        .await
        .expect("creating the webhook must succeed");
    assert!(!created.id.is_empty());
    let webhook_id = created.id;
    println!(
        "Created webhook: {} (url: {:?})",
        webhook_id, created.attributes.url
    );

    let detail = webhooks
        .get_details(&webhook_id, None)
        .await
        .expect("fetching the webhook must succeed");
    assert_eq!(detail.id, webhook_id);
    println!("Webhook url: {:?}", detail.attributes.url);

    let response = webhooks
        .delete(&webhook_id)
        .await
        .expect("deleting the webhook must succeed");
    assert_eq!(response.status_code, 204);
    println!("Deleted webhook: {}", webhook_id);
}

// =============================================================================
// Emails
// =============================================================================

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn emails_happy_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let emails = client.emails(&org_id);
    let email_events = client.email_events(&org_id);

    let response = emails
        .get_collection(None)
        .await
        .expect("listing emails must succeed");
    assert_eq!(response.status_code, 200);

    let path = common::document_path(common::FILE_NAME);
    let meta_data = common::build_email_meta_data();
    let created = emails
        .upload_and_create(
            &path,
            common::document_name(&path),
            true,
            Some(&meta_data),
            None,
        )
        .await
        .expect("creating the email must succeed");
    assert!(!created.id.is_empty());
    let email_id = created.id;
    println!(
        "Created email: {} (status: {:?})",
        email_id, created.attributes.status
    );

    let detail = emails
        .get_details(&email_id, None)
        .await
        .expect("fetching the email must succeed");
    assert_eq!(detail.id, email_id);
    println!("Email status: {:?}", detail.attributes.status);

    let events = email_events
        .get_collection(&email_id, None)
        .await
        .expect("listing email events must succeed");
    assert_eq!(events.status_code, 200);
    println!("Email events: {}", events.data.len());

    println!("sleep 5 seconds so the email reaches a retrievable state");
    sleep(Duration::from_secs(5)).await;

    let file_content = emails
        .get_file(&email_id)
        .await
        .expect("downloading the email file must succeed");
    assert!(!file_content.is_empty());
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn emails_cancel_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let emails = client.emails(&org_id);

    let path = common::document_path(common::FILE_NAME_CANCELLABLE);
    let meta_data = common::build_email_meta_data();
    let created = emails
        .upload_and_create(
            &path,
            common::document_name(&path),
            true,
            Some(&meta_data),
            None,
        )
        .await
        .expect("creating the email must succeed");
    assert!(!created.id.is_empty());
    let email_id = created.id;

    println!("sleep 10 seconds so the email reaches a cancellable state");
    sleep(Duration::from_secs(10)).await;

    let response = emails
        .cancel(&email_id)
        .await
        .expect("cancelling the email must succeed");
    assert_eq!(response.status_code, 202);
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn emails_delete_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let emails = client.emails(&org_id);

    let path = common::document_path(common::FILE_NAME);
    let meta_data = common::build_email_meta_data();
    let created = emails
        .upload_and_create(
            &path,
            common::document_name(&path),
            false,
            Some(&meta_data),
            None,
        )
        .await
        .expect("creating the email must succeed");
    assert!(!created.id.is_empty());
    let email_id = created.id;

    println!("sleep 10 seconds so the email reaches a deletable state");
    sleep(Duration::from_secs(10)).await;

    let response = emails
        .delete(&email_id)
        .await
        .expect("deleting the email must succeed");
    assert_eq!(response.status_code, 204);
    println!("Deleted email: {}", email_id);
}

// =============================================================================
// E-Bills
// =============================================================================

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn ebills_happy_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let ebills = client.ebills(&org_id);
    let ebill_events = client.ebill_events(&org_id);

    let response = ebills
        .get_collection(None)
        .await
        .expect("listing e-bills must succeed");
    assert_eq!(response.status_code, 200);

    let path = common::document_path(common::FILE_NAME);
    let Some(created) = create_ebill(&ebills, &path, false).await else {
        return;
    };
    assert!(!created.id.is_empty());
    let ebill_id = created.id;
    println!(
        "Created e-bill: {} (status: {:?})",
        ebill_id, created.attributes.status
    );

    let detail = ebills
        .get_details(&ebill_id, None)
        .await
        .expect("fetching the e-bill must succeed");
    assert_eq!(detail.id, ebill_id);
    println!("E-Bill status: {:?}", detail.attributes.status);

    let events = ebill_events
        .get_collection(&ebill_id, None)
        .await
        .expect("listing e-bill events must succeed");
    assert_eq!(events.status_code, 200);
    println!("E-Bill events: {}", events.data.len());

    println!("sleep 10 seconds so the e-bill reaches a retrievable state");
    sleep(Duration::from_secs(10)).await;

    let file_content = ebills
        .get_file(&ebill_id)
        .await
        .expect("downloading the e-bill file must succeed");
    assert!(!file_content.is_empty());
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn ebills_cancel_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let ebills = client.ebills(&org_id);

    let path = common::document_path(common::FILE_NAME_CANCELLABLE);
    let Some(created) = create_ebill(&ebills, &path, true).await else {
        return;
    };
    assert!(!created.id.is_empty());
    let ebill_id = created.id;

    println!("sleep 10 seconds so the e-bill reaches a cancellable state");
    sleep(Duration::from_secs(10)).await;

    let response = ebills
        .cancel(&ebill_id)
        .await
        .expect("cancelling the e-bill must succeed");
    assert_eq!(response.status_code, 202);
}

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn ebills_delete_case() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);
    let org_id = common::org_id(&client, &credentials).await;
    let ebills = client.ebills(&org_id);

    let path = common::document_path(common::FILE_NAME);
    let Some(created) = create_ebill(&ebills, &path, false).await else {
        return;
    };
    assert!(!created.id.is_empty());
    let ebill_id = created.id;

    println!("sleep 10 seconds so the e-bill reaches a deletable state");
    sleep(Duration::from_secs(10)).await;

    let response = ebills
        .delete(&ebill_id)
        .await
        .expect("deleting the e-bill must succeed");
    assert_eq!(response.status_code, 204);
    println!("Deleted e-bill: {}", ebill_id);
}

// =============================================================================
// User
// =============================================================================

#[tokio::test]
#[ignore = "integration: requires staging credentials in .env"]
async fn user_endpoints() {
    let Some(credentials) = common::require_credentials() else {
        return;
    };
    let client = common::client(&credentials);

    let user = client
        .users()
        .get_details(None)
        .await
        .expect("fetching the user must succeed");
    assert!(!user.id.is_empty());
    assert!(
        user.attributes
            .email
            .as_deref()
            .is_some_and(|email| !email.is_empty()),
        "The user must have an email attribute"
    );
    println!(
        "User: {:?} {:?} ({:?})",
        user.attributes.first_name, user.attributes.last_name, user.attributes.email
    );

    let associations = client
        .user_associations()
        .get_collection(None)
        .await
        .expect("listing user associations must succeed");
    assert_eq!(associations.status_code, 200);
    println!("User associations: {}", associations.data.len());
}
