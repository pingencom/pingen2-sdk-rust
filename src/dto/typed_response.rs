use super::json_api::{
    CollectionLinks, CollectionMeta, ItemLinks, JsonApiCollection, JsonApiResource,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ApiResource<A> {
    pub id: String,
    pub resource_type: String,
    pub attributes: A,
    pub relationships: Option<Value>,
    pub links: Option<ItemLinks>,
    pub meta: Option<Value>,
    pub included: Vec<Value>,
    pub status_code: u16,
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ApiCollection<A> {
    pub data: Vec<ApiCollectionItem<A>>,
    pub links: Option<CollectionLinks>,
    pub meta: Option<CollectionMeta>,
    pub included: Vec<Value>,
    pub status_code: u16,
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ApiCollectionItem<A> {
    pub id: String,
    pub resource_type: String,
    pub attributes: A,
    pub relationships: Option<Value>,
    pub links: Option<ItemLinks>,
    pub meta: Option<Value>,
}

impl crate::response::PingenResponse {
    pub fn to_resource<A: DeserializeOwned>(&self) -> crate::error::Result<ApiResource<A>> {
        let raw: JsonApiResource = serde_json::from_str(&self.body)?;
        let attributes: A = serde_json::from_value(raw.data.attributes)?;
        Ok(ApiResource {
            id: raw.data.id,
            resource_type: raw.data.resource_type,
            attributes,
            relationships: raw.data.relationships,
            links: raw.data.links,
            meta: raw.data.meta,
            included: raw.included,
            status_code: self.status_code,
            headers: self.headers.clone(),
        })
    }

    pub fn to_collection<A: DeserializeOwned>(&self) -> crate::error::Result<ApiCollection<A>> {
        let raw: JsonApiCollection = serde_json::from_str(&self.body)?;
        let data = raw
            .data
            .into_iter()
            .map(|item| {
                let attributes: A = serde_json::from_value(item.attributes)?;
                Ok(ApiCollectionItem {
                    id: item.id,
                    resource_type: item.resource_type,
                    attributes,
                    relationships: item.relationships,
                    links: item.links,
                    meta: item.meta,
                })
            })
            .collect::<std::result::Result<Vec<_>, serde_json::Error>>()?;
        Ok(ApiCollection {
            data,
            links: raw.links,
            meta: raw.meta,
            included: raw.included,
            status_code: self.status_code,
            headers: self.headers.clone(),
        })
    }
}

impl<A> ApiResource<A> {
    pub fn typed_relationships<R: DeserializeOwned>(&self) -> Option<R> {
        self.relationships
            .as_ref()
            .and_then(|r| serde_json::from_value(r.clone()).ok())
    }

    pub fn request_id(&self) -> Option<&str> {
        self.headers.get("x-request-id").map(|s| s.as_str())
    }
}

impl<A> ApiCollectionItem<A> {
    pub fn typed_relationships<R: DeserializeOwned>(&self) -> Option<R> {
        self.relationships
            .as_ref()
            .and_then(|r| serde_json::from_value(r.clone()).ok())
    }
}
