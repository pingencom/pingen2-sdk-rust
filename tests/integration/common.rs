//! Shared helpers, constants and credential loading for the integration tests.
//!
//! The integration tests hit the real Pingen **staging** API and therefore need
//! valid staging credentials. Credentials are read from a `.env` file at the
//! repository root (lines of `KEY=VALUE`, comments and blank lines are skipped,
//! surrounding quotes are stripped).
//!
//! Rust has no runtime "skip" for tests, so every integration test starts with
//! [`require_credentials`]: when no credentials are configured it prints a
//! `SKIPPED: ...` message and the test returns early (and therefore passes).

#![allow(dead_code)]

use pingen2_sdk::{EbillMetaData, EmailMetaData, PingenClient, API_STAGING};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// OAuth scope requested for the staging access token. Covers every resource
/// exercised by the integration suite.
pub const SCOPE: &str = "letter batch webhook organisation_read email ebill";

/// Document uploaded by the happy-case tests: `tests/fixtures/test.pdf`.
pub const FILE_NAME: &str = "test.pdf";

/// Document uploaded by the cancel/delete tests:
/// `tests/fixtures/test_simulate_cancellable.pdf`. The staging backend
/// recognises the `simulate_cancellable` name suffix and keeps such deliveries
/// in a cancellable state, which lets us exercise the cancel flow
/// deterministically.
pub const FILE_NAME_CANCELLABLE: &str = "test_simulate_cancellable.pdf";

/// Integration credentials for the staging API.
#[derive(Debug, Clone)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub organisation_id: String,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Hand-parsed `.env`: `KEY=VALUE` lines, comments/blank lines skipped and
/// surrounding single or double quotes stripped from values.
fn parse_dotenv(path: &Path) -> HashMap<String, String> {
    let mut values = HashMap::new();
    let Ok(contents) = std::fs::read_to_string(path) else {
        return values;
    };
    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = value.trim().trim_matches('"').trim_matches('\'');
        values.insert(key.trim().to_string(), value.to_string());
    }
    values
}

fn lookup(dotenv: &HashMap<String, String>, key: &str) -> String {
    std::env::var(key)
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| dotenv.get(key).cloned())
        .unwrap_or_default()
}

/// Loads the integration credentials.
pub fn load_credentials() -> Credentials {
    let dotenv = parse_dotenv(&repo_root().join(".env"));

    Credentials {
        client_id: lookup(&dotenv, "PINGEN2_CLIENT_ID"),
        client_secret: lookup(&dotenv, "PINGEN2_CLIENT_SECRET"),
        organisation_id: lookup(&dotenv, "PINGEN2_ORGANISATION_ID"),
    }
}

/// Returns the credentials, or `None` after printing a `SKIPPED` message when
/// client id / secret are not configured. Callers return early on `None`.
pub fn require_credentials() -> Option<Credentials> {
    let credentials = load_credentials();
    if credentials.client_id.is_empty() || credentials.client_secret.is_empty() {
        eprintln!(
            "SKIPPED: integration credentials not configured. Copy .env.example to .env \
             and fill in PINGEN2_CLIENT_ID / PINGEN2_CLIENT_SECRET."
        );
        return None;
    }
    Some(credentials)
}

/// API base for the suite. It must never run against production.
pub fn api_base() -> &'static str {
    API_STAGING
}

/// Builds a client that obtains and refreshes `client_credentials` tokens
/// itself. The suite must never run against production.
pub fn client(credentials: &Credentials) -> PingenClient {
    PingenClient::with_credentials_staging(
        credentials.client_id.clone(),
        credentials.client_secret.clone(),
        Some(SCOPE),
    )
}

/// The configured organisation id, or the first one returned by the API.
pub async fn org_id(client: &PingenClient, credentials: &Credentials) -> String {
    if !credentials.organisation_id.is_empty() {
        println!(
            "Using organisation ID from .env: {}",
            credentials.organisation_id
        );
        return credentials.organisation_id.clone();
    }

    let response = client
        .organisations()
        .get_collection(None)
        .await
        .expect("listing organisations must succeed");
    assert!(
        !response.data.is_empty(),
        "No organisations returned - check the staging credentials."
    );
    let org_id = response.data[0].id.clone();
    println!("Using first organisation ID: {}", org_id);
    org_id
}

/// Absolute path of a fixture document (see [`FILE_NAME`] and
/// [`FILE_NAME_CANCELLABLE`]).
pub fn document_path(file_name: &str) -> PathBuf {
    repo_root().join("tests/fixtures").join(file_name)
}

/// `file_original_name` derived from the uploaded file itself.
pub fn document_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .expect("document path must end in a valid UTF-8 file name")
}

pub fn build_email_meta_data() -> EmailMetaData {
    EmailMetaData {
        sender_name: "Pingen Test".to_string(),
        recipient_email: "grzegorz.morgas@pingen.com".to_string(),
        recipient_name: "Test Recipient".to_string(),
        reply_email: "noreply@example.com".to_string(),
        reply_name: "Reply Test".to_string(),
        subject: "Integration Test Email".to_string(),
        content: "Dear Recipient\\n\\nThis is an integration test.\\n\\nBest regards".to_string(),
    }
}

pub fn build_ebill_meta_data() -> EbillMetaData {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must not be before the UNIX epoch");
    let today = (now.as_secs() / 86_400) as i64;

    EbillMetaData {
        // Unique per call so repeated runs never clash on a duplicate invoice
        // number (no uuid dependency needed).
        invoice_number: format!("INV-{:x}", now.as_nanos()),
        invoice_date: iso_date_from_unix_days(today),
        invoice_due_date: iso_date_from_unix_days(today + 30),
        recipient_identifier: "41100000014283293".to_string(),
    }
}

/// Formats a day count since 1970-01-01 as an ISO `YYYY-MM-DD` date.
fn iso_date_from_unix_days(days: i64) -> String {
    let (year, month, day) = civil_from_days(days);
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Howard Hinnant's `civil_from_days` algorithm: converts days since the UNIX
/// epoch into a proleptic Gregorian (year, month, day).
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let shifted = if z >= 0 { z } else { z - 146_096 };
    let era = shifted / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32; // [1, 12]
    (if month <= 2 { y + 1 } else { y }, month, day)
}
