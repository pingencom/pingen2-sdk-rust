use crate::api::file_upload::FileUpload;
use crate::api::requestor::{ApiRequestor, TokenProvider};
use crate::dto::{ApiCollection, ApiResource, EbillAttributes, EbillMetaData, PresetRelationship};
use crate::error::Result;
use crate::response::PingenResponse;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

pub struct Ebills {
    org_id: String,
    requestor: ApiRequestor,
}
impl Ebills {
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
    pub async fn get_details(
        &self,
        ebill_id: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiResource<EbillAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!(
                    "/organisations/{}/deliveries/ebills/{}",
                    self.org_id, ebill_id
                ),
                params,
            )
            .await?;
        resp.to_resource()
    }
    pub async fn get_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<EbillAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!("/organisations/{}/deliveries/ebills", self.org_id),
                params,
            )
            .await?;
        resp.to_collection()
    }
    pub async fn upload_and_create(
        &self,
        file_path: &Path,
        file_original_name: &str,
        auto_send: bool,
        meta_data: Option<&EbillMetaData>,
        preset: Option<&PresetRelationship>,
    ) -> Result<ApiResource<EbillAttributes>> {
        let fu = FileUpload::new(&self.requestor);
        let (url, sig) = fu.request_file_upload().await?;
        fu.put_file(file_path, &url).await?;
        self.create(&url, &sig, file_original_name, auto_send, meta_data, preset)
            .await
    }
    pub async fn create(
        &self,
        file_url: &str,
        file_signature: &str,
        file_name: &str,
        auto_send: bool,
        meta_data: Option<&EbillMetaData>,
        preset: Option<&PresetRelationship>,
    ) -> Result<ApiResource<EbillAttributes>> {
        let mut attrs = json!({
            "file_original_name": file_name, "file_url": file_url,
            "file_url_signature": file_signature, "auto_send": auto_send
        });
        if let Some(md) = meta_data {
            attrs["meta_data"] = json!(md);
        }
        let mut data = json!({ "type": "ebills", "attributes": attrs });
        if let Some(p) = preset {
            data["relationships"] = p.to_value();
        }
        let payload = json!({ "data": data });
        let resp = self
            .requestor
            .post(
                &format!("/organisations/{}/deliveries/ebills", self.org_id),
                &payload.to_string(),
            )
            .await?;
        resp.to_resource()
    }

    pub async fn send(&self, ebill_id: &str) -> Result<ApiResource<EbillAttributes>> {
        let payload = json!({ "data": { "id": ebill_id, "type": "ebills" }});
        let resp = self
            .requestor
            .patch(
                &format!(
                    "/organisations/{}/deliveries/ebills/{}/send",
                    self.org_id, ebill_id
                ),
                Some(&payload.to_string()),
            )
            .await?;
        resp.to_resource()
    }

    pub async fn cancel(&self, ebill_id: &str) -> Result<PingenResponse> {
        self.requestor
            .patch(
                &format!(
                    "/organisations/{}/deliveries/ebills/{}/cancel",
                    self.org_id, ebill_id
                ),
                None,
            )
            .await
    }

    pub async fn delete(&self, ebill_id: &str) -> Result<PingenResponse> {
        self.requestor
            .delete(
                &format!(
                    "/organisations/{}/deliveries/ebills/{}",
                    self.org_id, ebill_id
                ),
                None,
            )
            .await
    }

    pub async fn get_file(&self, ebill_id: &str) -> Result<Vec<u8>> {
        self.requestor
            .download(&format!(
                "/organisations/{}/deliveries/ebills/{}/file",
                self.org_id, ebill_id
            ))
            .await
    }
}
