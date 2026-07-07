use crate::api::requestor::ApiRequestor;
use crate::dto::{ApiCollection, UserAssociationAttributes};
use crate::error::Result;
use std::collections::HashMap;

pub struct UserAssociations {
    requestor: ApiRequestor,
}
impl UserAssociations {
    pub fn new(access_token: impl Into<String>, api_base: impl Into<String>) -> Self {
        Self {
            requestor: ApiRequestor::new(access_token, api_base),
        }
    }
    pub async fn get_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<UserAssociationAttributes>> {
        let resp = self.requestor.get("/user/associations", params).await?;
        resp.to_collection()
    }
}
