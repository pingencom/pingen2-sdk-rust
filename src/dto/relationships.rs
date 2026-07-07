use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipItem {
    #[serde(default)]
    pub links: Option<RelationshipLinks>,
    #[serde(default)]
    pub data: Option<RelationshipData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipData {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipLinks {
    #[serde(default)]
    pub related: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipMany {
    #[serde(default)]
    pub links: Option<RelationshipLinks>,
}
