use crate::api::file_upload::FileUpload;
use crate::api::requestor::ApiRequestor;
use crate::dto::{
    ApiCollection, ApiResource, BatchAttributes, BatchStatisticsAttributes, PresetRelationship,
};
use crate::error::Result;
use crate::response::PingenResponse;
use crate::types::{
    AddressPosition, BatchDeliveryProduct, BatchIcon, GroupingType, PaperType, PrintMode,
    PrintSpectrum, SplitPosition, SplitType,
};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

pub struct Batches {
    org_id: String,
    requestor: ApiRequestor,
}

impl Batches {
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
        batch_id: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiResource<BatchAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!("/organisations/{}/batches/{}", self.org_id, batch_id),
                params,
            )
            .await?;
        resp.to_resource()
    }

    pub async fn get_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<BatchAttributes>> {
        let resp = self
            .requestor
            .get(&format!("/organisations/{}/batches", self.org_id), params)
            .await?;
        resp.to_collection()
    }

    pub async fn upload_and_create(
        &self,
        file_path: &Path,
        name: &str,
        icon: BatchIcon,
        file_original_name: &str,
        address_position: AddressPosition,
        grouping_type: GroupingType,
        split_type: SplitType,
        split_size: Option<i64>,
        split_separator: Option<&str>,
        split_position: Option<SplitPosition>,
        preset: Option<&PresetRelationship>,
    ) -> Result<ApiResource<BatchAttributes>> {
        let fu = FileUpload::new(&self.requestor);
        let (url, sig) = fu.request_file_upload().await?;
        fu.put_file(file_path, &url).await?;
        self.create(
            &url,
            &sig,
            name,
            icon,
            file_original_name,
            address_position,
            grouping_type,
            split_type,
            split_size,
            split_separator,
            split_position,
            preset,
        )
        .await
    }

    pub async fn create(
        &self,
        file_url: &str,
        file_url_signature: &str,
        name: &str,
        icon: BatchIcon,
        file_original_name: &str,
        address_position: AddressPosition,
        grouping_type: GroupingType,
        split_type: SplitType,
        split_size: Option<i64>,
        split_separator: Option<&str>,
        split_position: Option<SplitPosition>,
        preset: Option<&PresetRelationship>,
    ) -> Result<ApiResource<BatchAttributes>> {
        let mut attrs = json!({
            "file_url": file_url, "file_url_signature": file_url_signature,
            "name": name, "icon": icon.as_str(), "file_original_name": file_original_name,
            "address_position": address_position.as_str(), "grouping_type": grouping_type.as_str(),
            "grouping_options_split_type": split_type.as_str(),
        });
        if let Some(sz) = split_size {
            attrs["grouping_options_split_size"] = json!(sz);
        }
        if let Some(ss) = split_separator {
            attrs["grouping_options_split_separator"] = json!(ss);
        }
        if let Some(sp) = split_position {
            attrs["grouping_options_split_position"] = json!(sp.as_str());
        }
        let mut data = json!({ "type": "batches", "attributes": attrs });
        if let Some(p) = preset {
            data["relationships"] = p.to_value();
        }
        let payload = json!({ "data": data });
        let resp = self
            .requestor
            .post(
                &format!("/organisations/{}/batches", self.org_id),
                &payload.to_string(),
            )
            .await?;
        resp.to_resource()
    }

    pub async fn send(
        &self,
        batch_id: &str,
        delivery_products: &[BatchDeliveryProduct],
        print_mode: PrintMode,
        print_spectrum: PrintSpectrum,
    ) -> Result<ApiResource<BatchAttributes>> {
        let payload = json!({ "data": { "id": batch_id, "type": "batches",
            "attributes": { "delivery_products": delivery_products, "print_mode": print_mode.as_str(), "print_spectrum": print_spectrum.as_str() }}});
        let resp = self
            .requestor
            .patch(
                &format!("/organisations/{}/batches/{}/send", self.org_id, batch_id),
                Some(&payload.to_string()),
            )
            .await?;
        resp.to_resource()
    }

    pub async fn cancel(&self, batch_id: &str) -> Result<PingenResponse> {
        self.requestor
            .patch(
                &format!("/organisations/{}/batches/{}/cancel", self.org_id, batch_id),
                None,
            )
            .await
    }

    pub async fn delete(&self, batch_id: &str) -> Result<PingenResponse> {
        self.requestor
            .delete(&format!(
                "/organisations/{}/batches/{}",
                self.org_id, batch_id
            ))
            .await
    }

    pub async fn edit(
        &self,
        batch_id: &str,
        paper_types: &[PaperType],
    ) -> Result<ApiResource<BatchAttributes>> {
        let types: Vec<&str> = paper_types.iter().map(|pt| pt.as_str()).collect();
        let payload = json!({ "data": { "id": batch_id, "type": "batches", "attributes": { "paper_types": types }}});
        let resp = self
            .requestor
            .patch(
                &format!("/organisations/{}/batches/{}", self.org_id, batch_id),
                Some(&payload.to_string()),
            )
            .await?;
        resp.to_resource()
    }

    pub async fn get_statistics(
        &self,
        batch_id: &str,
    ) -> Result<ApiResource<BatchStatisticsAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!(
                    "/organisations/{}/batches/{}/statistics",
                    self.org_id, batch_id
                ),
                None,
            )
            .await?;
        resp.to_resource()
    }
}
