use crate::api::requestor::ApiRequestor;
use crate::dto::{ApiCollection, ApiResource, WebhookAttributes};
use crate::error::Result;
use crate::response::PingenResponse;
use crate::types::WebhookEventCategory;
use serde_json::json;
use std::collections::HashMap;

pub struct Webhooks {
    org_id: String,
    requestor: ApiRequestor,
}
impl Webhooks {
    pub fn new(
        org_id: impl Into<String>,
        access_token: impl Into<String>,
        api_base: impl Into<String>,
    ) -> Self {
        Self {
            org_id: org_id.into(),
            requestor: ApiRequestor::new(access_token, api_base),
        }
    }
    pub async fn get_details(
        &self,
        webhook_id: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiResource<WebhookAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!("/organisations/{}/webhooks/{}", self.org_id, webhook_id),
                params,
            )
            .await?;
        resp.to_resource()
    }
    pub async fn get_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<WebhookAttributes>> {
        let resp = self
            .requestor
            .get(&format!("/organisations/{}/webhooks", self.org_id), params)
            .await?;
        resp.to_collection()
    }
    pub async fn create(
        &self,
        event_category: WebhookEventCategory,
        url: &str,
        signing_key: &str,
    ) -> Result<ApiResource<WebhookAttributes>> {
        let payload = json!({ "data": { "type": "webhooks",
            "attributes": { "event_category": event_category.as_str(), "url": url, "signing_key": signing_key }}});
        let resp = self
            .requestor
            .post(
                &format!("/organisations/{}/webhooks", self.org_id),
                &payload.to_string(),
            )
            .await?;
        resp.to_resource()
    }
    pub async fn delete(&self, webhook_id: &str) -> Result<PingenResponse> {
        self.requestor
            .delete(&format!(
                "/organisations/{}/webhooks/{}",
                self.org_id, webhook_id
            ))
            .await
    }
}
