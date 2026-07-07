#![allow(clippy::too_many_arguments)]

pub mod api;
pub mod dto;
pub mod error;
pub mod oauth;
pub mod response;
pub mod types;
pub mod webhook;

pub use api::*;
pub use dto::*;
pub use error::{PingenError, Result};
pub use oauth::{OAuth, TokenManager};
pub use response::PingenResponse;
pub use types::*;
pub use webhook::{IncomingWebhook, WebhookEvent};

pub const API_PRODUCTION: &str = "https://api.pingen.com";
pub const AUTH_PRODUCTION: &str = "https://identity.pingen.com";
pub const API_STAGING: &str = "https://api-staging.pingen.com";
pub const AUTH_STAGING: &str = "https://identity-staging.pingen.com";
