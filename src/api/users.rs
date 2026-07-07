use crate::api::requestor::ApiRequestor;
use crate::dto::{ApiResource, UserAttributes};
use crate::error::Result;
use std::collections::HashMap;

pub struct Users {
    requestor: ApiRequestor,
}
impl Users {
    pub fn new(access_token: impl Into<String>, api_base: impl Into<String>) -> Self {
        Self {
            requestor: ApiRequestor::new(access_token, api_base),
        }
    }
    pub async fn get_details(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiResource<UserAttributes>> {
        let resp = self.requestor.get("/user", params).await?;
        resp.to_resource()
    }
}
