use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct JsonApiResource {
    pub data: ResourceObject,
    #[serde(default)]
    pub included: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JsonApiCollection {
    pub data: Vec<ResourceObject>,
    #[serde(default)]
    pub links: Option<CollectionLinks>,
    #[serde(default)]
    pub meta: Option<CollectionMeta>,
    #[serde(default)]
    pub included: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResourceObject {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Value,
    #[serde(default)]
    pub relationships: Option<Value>,
    #[serde(default)]
    pub links: Option<ItemLinks>,
    #[serde(default)]
    pub meta: Option<Value>,
}

impl ResourceObject {
    pub fn typed_attributes<T: DeserializeOwned>(&self) -> Option<T> {
        serde_json::from_value(self.attributes.clone()).ok()
    }

    pub fn typed_relationships<T: DeserializeOwned>(&self) -> Option<T> {
        self.relationships
            .as_ref()
            .and_then(|r| serde_json::from_value(r.clone()).ok())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectionLinks {
    #[serde(default)]
    pub first: Option<String>,
    #[serde(default)]
    pub last: Option<String>,
    #[serde(default)]
    pub prev: Option<String>,
    #[serde(default)]
    pub next: Option<String>,
    #[serde(rename = "self", default)]
    pub self_link: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectionMeta {
    #[serde(default)]
    pub current_page: Option<u32>,
    #[serde(default)]
    pub last_page: Option<u32>,
    #[serde(default)]
    pub per_page: Option<u32>,
    #[serde(default)]
    pub from: Option<u32>,
    #[serde(default)]
    pub to: Option<u32>,
    #[serde(default)]
    pub total: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemLinks {
    #[serde(rename = "self", default)]
    pub self_link: Option<String>,
}
