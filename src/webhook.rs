use crate::dto::JsonApiResource;
use crate::error::{PingenError, Result};
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub struct WebhookEvent {
    pub body: String,
    pub data: Option<Value>,
}

impl WebhookEvent {
    pub fn as_resource(&self) -> Option<JsonApiResource> {
        self.data
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn event_type(&self) -> Option<&str> {
        self.data.as_ref().and_then(|v| v["data"]["type"].as_str())
    }
}

pub struct IncomingWebhook;

impl IncomingWebhook {
    pub fn construct_event(payload: &str, signature: &str, secret: &str) -> Result<WebhookEvent> {
        Self::verify_signature(payload, signature, secret)?;
        let data = serde_json::from_str(payload).ok();
        Ok(WebhookEvent {
            body: payload.to_string(),
            data,
        })
    }

    pub fn verify_signature(payload: &str, signature: &str, secret: &str) -> Result<()> {
        if signature.is_empty() {
            return Err(PingenError::WebhookSignature("Signature missing.".into()));
        }
        let signature_bytes = hex::decode(signature).map_err(|_| {
            PingenError::WebhookSignature("Webhook signature matching failed.".into())
        })?;
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| PingenError::WebhookSignature(e.to_string()))?;
        mac.update(payload.as_bytes());
        mac.verify_slice(&signature_bytes).map_err(|_| {
            PingenError::WebhookSignature("Webhook signature matching failed.".into())
        })?;
        Ok(())
    }
}
