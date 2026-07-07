use super::relationships::{RelationshipItem, RelationshipMany};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterRecipient {
    pub name: String,
    #[serde(default)]
    pub street: Option<String>,
    #[serde(default)]
    pub pobox: Option<String>,
    #[serde(default)]
    pub number: Option<String>,
    pub zip: String,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterSender {
    pub name: String,
    #[serde(default)]
    pub street: Option<String>,
    #[serde(default)]
    pub pobox: Option<String>,
    #[serde(default)]
    pub number: Option<String>,
    pub zip: String,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterMetaData {
    pub recipient: LetterRecipient,
    pub sender: LetterSender,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FontInfo {
    pub name: String,
    pub is_embedded: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LetterAttributes {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub file_original_name: Option<String>,
    #[serde(default)]
    pub file_pages: Option<u32>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub address_position: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub delivery_product: Option<String>,
    #[serde(default)]
    pub print_mode: Option<String>,
    #[serde(default)]
    pub print_spectrum: Option<String>,
    #[serde(default)]
    pub price_currency: Option<String>,
    #[serde(default)]
    pub price_value: Option<f64>,
    #[serde(default)]
    pub paper_types: Option<Vec<String>>,
    #[serde(default)]
    pub fonts: Option<Vec<FontInfo>>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub tracking_number: Option<String>,
    #[serde(default)]
    pub submitted_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LetterRelationships {
    #[serde(default)]
    pub organisation: Option<RelationshipItem>,
    #[serde(default)]
    pub events: Option<RelationshipMany>,
    #[serde(default)]
    pub batch: Option<RelationshipItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LetterPriceAttributes {
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub price: Option<f64>,
}
