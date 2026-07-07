use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PingenResponse {
    pub body: String,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub data: Option<Value>,
}

impl PingenResponse {
    pub fn new(body: String, status_code: u16, headers: HashMap<String, String>) -> Self {
        let data = if body.is_empty() {
            None
        } else {
            serde_json::from_str(&body).ok()
        };
        Self {
            body,
            status_code,
            headers,
            data,
        }
    }
}
