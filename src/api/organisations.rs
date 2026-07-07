use crate::api::requestor::ApiRequestor;
use crate::dto::{ApiCollection, ApiResource, OrganisationAttributes};
use crate::error::Result;
use std::collections::HashMap;

pub struct Organisations {
    requestor: ApiRequestor,
}
impl Organisations {
    pub fn new(access_token: impl Into<String>, api_base: impl Into<String>) -> Self {
        Self {
            requestor: ApiRequestor::new(access_token, api_base),
        }
    }
    pub async fn get_details(
        &self,
        org_id: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiResource<OrganisationAttributes>> {
        let resp = self
            .requestor
            .get(&format!("/organisations/{}", org_id), params)
            .await?;
        resp.to_resource()
    }
    pub async fn get_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<OrganisationAttributes>> {
        let resp = self.requestor.get("/organisations", params).await?;
        resp.to_collection()
    }
}
