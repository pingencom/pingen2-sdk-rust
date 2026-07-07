use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookAttributes {
    #[serde(default)]
    pub event_category: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub signing_key: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}
