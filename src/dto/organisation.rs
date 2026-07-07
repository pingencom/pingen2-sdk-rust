use super::relationships::RelationshipMany;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OrganisationAttributes {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub billing_mode: Option<String>,
    #[serde(default)]
    pub billing_currency: Option<String>,
    #[serde(default)]
    pub billing_balance: Option<f64>,
    #[serde(default)]
    pub missing_credits: Option<f64>,
    #[serde(default)]
    pub edition: Option<String>,
    #[serde(default)]
    pub default_country: Option<String>,
    #[serde(default)]
    pub default_address_position: Option<String>,
    #[serde(default)]
    pub data_retention_addresses: Option<u32>,
    #[serde(default)]
    pub data_retention_pdf: Option<u32>,
    #[serde(default)]
    pub limits_monthly_letters_count: Option<u32>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub flags: Option<Vec<String>>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrganisationRelationships {
    #[serde(default)]
    pub associations: Option<RelationshipMany>,
}
