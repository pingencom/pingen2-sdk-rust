use super::relationships::RelationshipItem;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserAttributes {
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub edition: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserAssociationAttributes {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserAssociationRelationships {
    #[serde(default)]
    pub organisation: Option<RelationshipItem>,
    #[serde(default)]
    pub user: Option<RelationshipItem>,
}
