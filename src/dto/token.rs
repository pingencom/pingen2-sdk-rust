use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub token_type: String,
    pub expires_in: u64,
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
}
