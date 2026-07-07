use super::relationships::RelationshipItem;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CorrectedAddress {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub street: Option<String>,
    #[serde(default)]
    pub number: Option<String>,
    #[serde(default)]
    pub zip: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookEventAttributes {
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub corrected_address: Option<CorrectedAddress>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookEventRelationships {
    #[serde(default)]
    pub organisation: Option<RelationshipItem>,
    #[serde(default)]
    pub letter: Option<RelationshipItem>,
    #[serde(default)]
    pub event: Option<RelationshipItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookChannelSubscriptionAttributes {
    #[serde(default)]
    pub identifier: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub approved_at: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookChannelSubscriptionRelationships {
    #[serde(default)]
    pub organisation: Option<RelationshipItem>,
    #[serde(default)]
    pub channel_ebill: Option<RelationshipItem>,
}
