use crate::api::requestor::ApiRequestor;
use crate::dto::{ApiCollection, EventAttributes};
use crate::error::Result;
use std::collections::HashMap;

pub struct LetterEvents {
    org_id: String,
    requestor: ApiRequestor,
}
impl LetterEvents {
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
        letter_id: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<EventAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!(
                    "/organisations/{}/deliveries/letters/{}/events",
                    self.org_id, letter_id
                ),
                params,
            )
            .await?;
        resp.to_collection()
    }
    pub async fn get_issue_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<EventAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!(
                    "/organisations/{}/deliveries/letters/events/issues",
                    self.org_id
                ),
                params,
            )
            .await?;
        resp.to_collection()
    }
    pub async fn get_undeliverable_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<EventAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!(
                    "/organisations/{}/deliveries/letters/events/undeliverable",
                    self.org_id
                ),
                params,
            )
            .await?;
        resp.to_collection()
    }
    pub async fn get_delivered_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<EventAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!(
                    "/organisations/{}/deliveries/letters/events/delivered",
                    self.org_id
                ),
                params,
            )
            .await?;
        resp.to_collection()
    }
    pub async fn get_sent_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<EventAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!(
                    "/organisations/{}/deliveries/letters/events/sent",
                    self.org_id
                ),
                params,
            )
            .await?;
        resp.to_collection()
    }
}
