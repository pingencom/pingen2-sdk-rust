# pingen2-sdk

Official Rust SDK for the [Pingen v2 API](https://api.pingen.com).

![CI](https://github.com/pingencom/pingen2-sdk-rust/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/pingen2-sdk)](https://crates.io/crates/pingen2-sdk)

---

## Requirements

- Rust 1.97+ (2021 edition)
- Tokio async runtime
- A Pingen account with OAuth credentials ([how to obtain](https://api.pingen.com/documentation#section/Authentication/How-to-obtain-a-Client-ID))

---

## Installation

```toml
[dependencies]
pingen2-sdk = "x.x.x"
tokio       = { version = "1", features = ["full"] }
```

---

## Quick start

```rust,no_run
use pingen2_sdk::{
    PingenClient, PresetRelationship,
    AddressPosition, DeliveryProduct, PrintMode, PrintSpectrum,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a client that fetches and refreshes access tokens for you
    //    (client_credentials grant, tokens are cached and renewed automatically)
    let client = PingenClient::with_credentials(
        "CLIENT_ID", "CLIENT_SECRET", Some("letter organisation_read"),
    );

    // 2. List organisations
    let orgs = client.organisations().get_collection(None).await?;
    let org_id = &orgs.data[0].id;
    println!("Org: {} - {:?}", org_id, orgs.data[0].attributes.name);

    // 3. Upload and create a letter
    let preset = PresetRelationship::new("YOUR_PRESET_ID");
    let letter = client.letters(org_id)
        .upload_and_create(
            Path::new("test.pdf"),
            "invoice.pdf",
            AddressPosition::Left,
            false,
            Some(DeliveryProduct::Fast),
            Some(PrintMode::Simplex),
            Some(PrintSpectrum::Color),
            None,
            Some(&preset),
        )
        .await?;
    println!("Letter created: {}", letter.id);

    // 4. Send the letter
    let sent = client.letters(org_id)
        .send(&letter.id, DeliveryProduct::Fast, PrintMode::Simplex, PrintSpectrum::Color)
        .await?;
    println!("Sent (status: {})", sent.status_code);

    Ok(())
}
```

For a complete, runnable walk-through of every resource (letters, batches,
emails, e-bills, webhooks, events, users) see the end-to-end integration suite in
[`tests/integration/pingen.rs`](tests/integration/pingen.rs).

---

## Environments

```rust,no_run
use pingen2_sdk::{Letters, PingenClient, API_PRODUCTION};

fn main() {
    let access_token = "ACCESS_TOKEN";

    // Production (default)
    let _client = PingenClient::new(access_token);

    // Staging
    let _client = PingenClient::new_staging(access_token);

    // Or use API structs directly with a custom base URL
    let _letters = Letters::new("org-id", "token", API_PRODUCTION);
}
```

---

## Authentication

### Automatic token management (recommended)

`PingenClient::with_credentials` (and `with_credentials_staging`) builds a
`TokenManager` internally: the first request fetches a `client_credentials`
access token, caches it, and every subsequent request reuses it until it is
about to expire — tokens are refreshed 300 seconds before their reported
expiry, so a token never dies mid-request.

```rust,no_run
use pingen2_sdk::PingenClient;

fn main() {
    let _client = PingenClient::with_credentials("CLIENT_ID", "CLIENT_SECRET", None);

    // Optionally restrict the token to a scope:
    let _client = PingenClient::with_credentials(
        "CLIENT_ID",
        "CLIENT_SECRET",
        Some("letter batch webhook organisation_read"),
    );
}
```

You can also use `TokenManager` directly — every API struct accepts anything
convertible into a `TokenProvider` (`&str`, `String`, `TokenManager`,
`Arc<TokenManager>`):

```rust,no_run
use pingen2_sdk::{Letters, PingenClient, TokenManager, API_PRODUCTION};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Arc::new(TokenManager::client_credentials(
        API_PRODUCTION,
        "CLIENT_ID",
        "CLIENT_SECRET",
        None,
    ));

    // Peek at the current token (fetched/refreshed on demand):
    let _token = manager.get_access_token().await?;

    // Share the same manager between a client and standalone API structs:
    let _client = PingenClient::new(manager.clone());
    let _letters = Letters::new("org-id", manager.clone(), API_PRODUCTION);

    // Drop the cached token and force a fresh fetch on the next request:
    manager.invalidate().await;

    Ok(())
}
```

### Static access token

If you already have an access token (e.g. from the authorization code or
implicit flow), pass it directly:

```rust,no_run
use pingen2_sdk::PingenClient;

fn main() {
    let _client = PingenClient::new("ACCESS_TOKEN");
}
```

### Low-level OAuth

```rust,no_run
use pingen2_sdk::{OAuth, API_PRODUCTION};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Client credentials
    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());
    let token =
        OAuth::get_token(API_PRODUCTION, Some("CLIENT_ID"), Some("CLIENT_SECRET"), params).await?;
    let _access_token = token["access_token"].as_str().unwrap();

    // Authorization code flow
    let _url = OAuth::authorize_url(false, Some("CLIENT_ID"), HashMap::new())?;
    // redirect user to `url`, then exchange the code:
    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "authorization_code".to_string());
    params.insert("code".to_string(), "AUTH_CODE".to_string());
    params.insert(
        "redirect_uri".to_string(),
        "https://myapp.com/callback".to_string(),
    );
    let _token =
        OAuth::get_token(API_PRODUCTION, Some("CLIENT_ID"), Some("CLIENT_SECRET"), params).await?;

    // Implicit flow (parse fragment)
    let _params = OAuth::get_token_from_implicit("access_token=abc&expires_in=3600");

    Ok(())
}
```

---

## Webhook signature verification

```rust,no_run
use pingen2_sdk::IncomingWebhook;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payload = "{}";
    let signature = "signature-header-value";
    let secret = "your-signing-secret";

    let event = IncomingWebhook::construct_event(payload, signature, secret)?;

    // Event type: "issues", "sent", "undeliverable", "delivered", "channel_subscriptions"
    println!("Type: {:?}", event.event_type());

    // Access the raw JSON data
    if let Some(resource) = event.as_resource() {
        println!("Resource ID: {}", resource.data.id);
    }

    Ok(())
}
```

---

## Available API modules

| Module | Methods |
|--------|---------|
| `Letters` | `get_details`, `get_collection`, `create`, `upload_and_create`, `send`, `cancel`, `delete`, `edit`, `get_file`, `calculate_price` |
| `LetterEvents` | `get_collection`, `get_issue_collection`, `get_undeliverable_collection`, `get_delivered_collection`, `get_sent_collection` |
| `Batches` | `get_details`, `get_collection`, `create`, `upload_and_create` (incl. `channel_type`), `send_post`, `send_email`, `send_ebill`, `update`, `cancel`, `delete` (`with_deliverables`), `edit`, `get_statistics` |
| `BatchEvents` | `get_collection` |
| `Ebills` | `get_details`, `get_collection`, `create`, `upload_and_create`, `send`, `cancel`, `delete`, `get_file` |
| `EbillEvents` | `get_collection` |
| `Emails` | `get_details`, `get_collection`, `create`, `upload_and_create`, `cancel`, `delete`, `get_file` |
| `EmailEvents` | `get_collection` |
| `Organisations` | `get_details`, `get_collection` |
| `Users` | `get_details` |
| `UserAssociations` | `get_collection` |
| `Webhooks` | `get_details`, `get_collection`, `create`, `delete` |

---

## Development

### Docker (recommended)

```sh
docker compose build
docker compose up -d

# Tests
docker compose exec rust-sdk cargo test

# Formatting & linting
docker compose exec rust-sdk cargo fmt -- --check
docker compose exec rust-sdk cargo clippy --tests --examples -- -D warnings

# Code coverage
docker compose exec rust-sdk cargo tarpaulin --out Stdout --skip-clean
```

### Locally

```sh
cargo test
cargo fmt -- --check
cargo clippy --tests --examples -- -D warnings
cargo install cargo-tarpaulin && cargo tarpaulin --out Stdout
```

### Integration tests

The integration test suite runs against the **staging** environment only and
needs real staging credentials. The tests are marked `#[ignore]`, so they never
run as part of the normal `cargo test` or in CI.

```sh
cp .env.example .env   # then fill in the PINGEN2_* values
cargo test --test integration_pingen --test integration_oauth -- --ignored --test-threads=1
```

Or inside the Docker container:

```sh
docker compose exec rust-sdk cargo test --test integration_pingen --test integration_oauth -- --ignored --test-threads=1
```

### Changing the Rust version

```sh
RUST_VERSION=1.98 docker compose build
```

---

## API documentation

https://api.pingen.com/documentation

---

## License

MIT -- see [LICENSE](LICENSE)
