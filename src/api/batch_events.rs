use crate::api::requestor::ApiRequestor;
use crate::dto::{ApiCollection, EventAttributes};
use crate::error::Result;

use std::collections::HashMap;

pub struct BatchEvents {
    org_id: String,
    requestor: ApiRequestor,
}
impl BatchEvents {
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
    pub async fn get_collection(
        &self,
        batch_id: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<EventAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!("/organisations/{}/batches/{}/events", self.org_id, batch_id),
                params,
            )
            .await?;
        resp.to_collection()
    }
}
