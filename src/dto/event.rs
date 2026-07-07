use super::relationships::RelationshipItem;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct EventAttributes {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub producer: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub has_image: Option<bool>,
    #[serde(default)]
    pub data: Option<Vec<Value>>,
    #[serde(default)]
    pub emitted_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventRelationships {
    #[serde(default)]
    pub letter: Option<RelationshipItem>,
    #[serde(default)]
    pub batch: Option<RelationshipItem>,
}
