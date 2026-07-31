use crate::api::requestor::{ApiRequestor, TokenProvider};
use crate::dto::{ApiCollection, EventAttributes};
use crate::error::Result;
use std::collections::HashMap;

pub struct EmailEvents {
    org_id: String,
    requestor: ApiRequestor,
}
impl EmailEvents {
    pub fn new(
        org_id: impl Into<String>,
        access_token: impl Into<TokenProvider>,
        api_base: impl Into<String>,
    ) -> Self {
        Self {
            org_id: org_id.into(),
            requestor: ApiRequestor::new(access_token, api_base),
        }
    }
    pub async fn get_collection(
        &self,
        email_id: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<EventAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!(
                    "/organisations/{}/deliveries/emails/{}/events",
                    self.org_id, email_id
                ),
                params,
            )
            .await?;
        resp.to_collection()
    }
}
