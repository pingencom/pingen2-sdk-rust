use super::relationships::{RelationshipItem, RelationshipMany};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct BatchAttributes {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub file_original_name: Option<String>,
    #[serde(default)]
    pub letter_count: Option<u32>,
    #[serde(default)]
    pub address_position: Option<String>,
    #[serde(default)]
    pub price_currency: Option<String>,
    #[serde(default)]
    pub price_value: Option<f64>,
    #[serde(default)]
    pub print_mode: Option<String>,
    #[serde(default)]
    pub print_spectrum: Option<String>,
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
pub struct BatchRelationships {
    #[serde(default)]
    pub organisation: Option<RelationshipItem>,
    #[serde(default)]
    pub events: Option<RelationshipMany>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchStatisticsAttributes {
    #[serde(default)]
    pub letter_validating: Option<u32>,
    #[serde(default)]
    pub letter_groups: Option<Vec<Value>>,
    #[serde(default)]
    pub letter_countries: Option<Vec<Value>>,
}
