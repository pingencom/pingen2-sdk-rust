use super::relationships::{RelationshipItem, RelationshipMany};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMetaData {
    pub sender_name: String,
    pub recipient_email: String,
    pub recipient_name: String,
    pub reply_email: String,
    pub reply_name: String,
    pub subject: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailAttributes {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub file_original_name: Option<String>,
    #[serde(default)]
    pub file_pages: Option<u32>,
    #[serde(default)]
    pub recipient_identifier: Option<String>,
    #[serde(default)]
    pub price_currency: Option<String>,
    #[serde(default)]
    pub price_value: Option<f64>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub submitted_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailRelationships {
    #[serde(default)]
    pub organisation: Option<RelationshipItem>,
    #[serde(default)]
    pub events: Option<RelationshipMany>,
}
