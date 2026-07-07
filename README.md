# pingen2-sdk

Official Rust SDK for the [Pingen v2 API](https://api.pingen.com).

![CI](https://github.com/pingencom/pingen2-sdk-rust/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/pingen2-sdk)](https://crates.io/crates/pingen2-sdk)

---

## Requirements

- Rust 1.94+ (2021 edition)
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

```rust
use pingen2_sdk::{
    PingenClient, OAuth, PresetRelationship,
    AddressPosition, DeliveryProduct, PrintMode, PrintSpectrum,
    API_PRODUCTION,
};
use std::collections::HashMap;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Authenticate
    let mut params = HashMap::new();
    params.insert("grant_type".to_string(), "client_credentials".to_string());
    let token = OAuth::get_token(
        API_PRODUCTION, Some("CLIENT_ID"), Some("CLIENT_SECRET"), params,
    ).await?;
    let access_token = token["access_token"].as_str().unwrap();

    // 2. Create a client
    let client = PingenClient::new(access_token);

    // 3. List organisations
    let orgs = client.organisations().get_collection(None).await?;
    let org_id = &orgs.data[0].id;
    println!("Org: {} - {:?}", org_id, orgs.data[0].attributes.name);

    // 4. Upload and create a letter
    let preset = PresetRelationship::new("YOUR_PRESET_ID");
    let letter = client.letters(org_id)
        .upload_and_create(
            Path::new("letter.pdf"),
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

    // 5. Send the letter
    let sent = client.letters(org_id)
        .send(&letter.id, DeliveryProduct::Fast, PrintMode::Simplex, PrintSpectrum::Color)
        .await?;
    println!("Sent (status: {})", sent.status_code);

    Ok(())
}
```

See [`examples/basic_usage.rs`](examples/basic_usage.rs) for a more complete example.

---

## Environments

```rust
use pingen2_sdk::PingenClient;

// Production (default)
let client = PingenClient::new(access_token);

// Staging
let client = PingenClient::new_staging(access_token);

// Or use API structs directly with a custom base URL
use pingen2_sdk::{Letters, API_PRODUCTION};
let letters = Letters::new("org-id", "token", API_PRODUCTION);
```

---

## OAuth

```rust
use pingen2_sdk::{OAuth, API_PRODUCTION};
use std::collections::HashMap;

// Client credentials
let mut params = HashMap::new();
params.insert("grant_type".to_string(), "client_credentials".to_string());
let token = OAuth::get_token(
    API_PRODUCTION, Some("CLIENT_ID"), Some("CLIENT_SECRET"), params,
).await?;
let access_token = token["access_token"].as_str().unwrap();

// Authorization code flow
let url = OAuth::authorize_url(false, Some("CLIENT_ID"), HashMap::new())?;
// redirect user to `url`, then exchange the code:
let mut params = HashMap::new();
params.insert("grant_type".to_string(), "authorization_code".to_string());
params.insert("code".to_string(), "AUTH_CODE".to_string());
params.insert("redirect_uri".to_string(), "https://myapp.com/callback".to_string());
let token = OAuth::get_token(
    API_PRODUCTION, Some("CLIENT_ID"), Some("CLIENT_SECRET"), params,
).await?;

// Implicit flow (parse fragment)
let params = OAuth::get_token_from_implicit("access_token=abc&expires_in=3600");
```

---

## Webhook signature verification

```rust
use pingen2_sdk::IncomingWebhook;

let event = IncomingWebhook::construct_event(payload, signature, secret)?;

// Event type: "issues", "sent", "undeliverable", "delivered", "channel_subscriptions"
println!("Type: {:?}", event.event_type());

// Access the raw JSON data
if let Some(resource) = event.as_resource() {
    println!("Resource ID: {}", resource.data.id);
}
```

---

## Available API modules

| Module | Methods |
|--------|---------|
| `Letters` | `get_details`, `get_collection`, `create`, `upload_and_create`, `send`, `cancel`, `delete`, `edit`, `get_file`, `calculate_price` |
| `LetterEvents` | `get_collection`, `get_issue_collection`, `get_undeliverable_collection`, `get_delivered_collection`, `get_sent_collection` |
| `Batches` | `get_details`, `get_collection`, `create`, `upload_and_create`, `send`, `cancel`, `delete`, `edit`, `get_statistics` |
| `BatchEvents` | `get_collection` |
| `Ebills` | `get_details`, `get_collection`, `create`, `upload_and_create` |
| `Emails` | `get_details`, `get_collection`, `create`, `upload_and_create` |
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

### Changing the Rust version

```sh
RUST_VERSION=1.95 docker compose build
```

---

## API documentation

https://api.pingen.com/documentation

---

## License

MIT -- see [LICENSE](LICENSE)
