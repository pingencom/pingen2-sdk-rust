use crate::api::file_upload::FileUpload;
use crate::api::requestor::ApiRequestor;
use crate::dto::{
    ApiCollection, ApiResource, LetterAttributes, LetterMetaData, LetterPriceAttributes,
    PresetRelationship,
};
use crate::error::{PingenError, Result};
use crate::response::PingenResponse;
use crate::types::{AddressPosition, DeliveryProduct, PaperType, PrintMode, PrintSpectrum};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

pub struct Letters {
    org_id: String,
    requestor: ApiRequestor,
}

impl Letters {
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
        letter_id: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiResource<LetterAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!(
                    "/organisations/{}/deliveries/letters/{}",
                    self.org_id, letter_id
                ),
                params,
            )
            .await?;
        resp.to_resource()
    }

    pub async fn get_collection(
        &self,
        params: Option<&HashMap<String, String>>,
    ) -> Result<ApiCollection<LetterAttributes>> {
        let resp = self
            .requestor
            .get(
                &format!("/organisations/{}/deliveries/letters", self.org_id),
                params,
            )
            .await?;
        resp.to_collection()
    }

    fn validate_auto_send(
        auto_send: bool,
        delivery_product: &Option<DeliveryProduct>,
        print_mode: &Option<PrintMode>,
        print_spectrum: &Option<PrintSpectrum>,
    ) -> Result<()> {
        if auto_send
            && (delivery_product.is_none() || print_mode.is_none() || print_spectrum.is_none())
        {
            return Err(PingenError::Validation(
                "When auto_send is true, delivery_product, print_mode and print_spectrum are required".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn upload_and_create(
        &self,
        file_path: &Path,
        file_original_name: &str,
        address_position: AddressPosition,
        auto_send: bool,
        delivery_product: Option<DeliveryProduct>,
        print_mode: Option<PrintMode>,
        print_spectrum: Option<PrintSpectrum>,
        meta_data: Option<&LetterMetaData>,
        preset: Option<&PresetRelationship>,
    ) -> Result<ApiResource<LetterAttributes>> {
        Self::validate_auto_send(auto_send, &delivery_product, &print_mode, &print_spectrum)?;
        let fu = FileUpload::new(&self.requestor);
        let (url, sig) = fu.request_file_upload().await?;
        fu.put_file(file_path, &url).await?;
        self.create(
            &url,
            &sig,
            file_original_name,
            address_position,
            auto_send,
            delivery_product,
            print_mode,
            print_spectrum,
            meta_data,
            preset,
        )
        .await
    }

    pub async fn create(
        &self,
        file_url: &str,
        file_signature: &str,
        file_original_name: &str,
        address_position: AddressPosition,
        auto_send: bool,
        delivery_product: Option<DeliveryProduct>,
        print_mode: Option<PrintMode>,
        print_spectrum: Option<PrintSpectrum>,
        meta_data: Option<&LetterMetaData>,
        preset: Option<&PresetRelationship>,
    ) -> Result<ApiResource<LetterAttributes>> {
        Self::validate_auto_send(auto_send, &delivery_product, &print_mode, &print_spectrum)?;

        let mut attributes = json!({
            "file_original_name": file_original_name,
            "file_url": file_url,
            "file_url_signature": file_signature,
            "address_position": address_position.as_str(),
            "auto_send": auto_send,
        });
        if let Some(dp) = delivery_product {
            attributes["delivery_product"] = json!(dp.as_str());
        }
        if let Some(pm) = print_mode {
            attributes["print_mode"] = json!(pm.as_str());
        }
        if let Some(ps) = print_spectrum {
            attributes["print_spectrum"] = json!(ps.as_str());
        }
        if let Some(md) = meta_data {
            attributes["meta_data"] = json!(md);
        }

        let mut data = json!({ "type": "letters", "attributes": attributes });
        if let Some(p) = preset {
            data["relationships"] = p.to_value();
        }
        let payload = json!({ "data": data });
        let resp = self
            .requestor
            .post(
                &format!("/organisations/{}/deliveries/letters", self.org_id),
                &payload.to_string(),
            )
            .await?;
        resp.to_resource()
    }

    pub async fn send(
        &self,
        letter_id: &str,
        delivery_product: DeliveryProduct,
        print_mode: PrintMode,
        print_spectrum: PrintSpectrum,
    ) -> Result<ApiResource<LetterAttributes>> {
        let payload = json!({ "data": { "id": letter_id, "type": "letters",
            "attributes": { "delivery_product": delivery_product.as_str(), "print_mode": print_mode.as_str(), "print_spectrum": print_spectrum.as_str() }}});
        let resp = self
            .requestor
            .patch(
                &format!(
                    "/organisations/{}/deliveries/letters/{}/send",
                    self.org_id, letter_id
                ),
                Some(&payload.to_string()),
            )
            .await?;
        resp.to_resource()
    }

    pub async fn cancel(&self, letter_id: &str) -> Result<PingenResponse> {
        self.requestor
            .patch(
                &format!(
                    "/organisations/{}/deliveries/letters/{}/cancel",
                    self.org_id, letter_id
                ),
                None,
            )
            .await
    }

    pub async fn delete(&self, letter_id: &str) -> Result<PingenResponse> {
        self.requestor
            .delete(&format!(
                "/organisations/{}/deliveries/letters/{}",
                self.org_id, letter_id
            ))
            .await
    }

    pub async fn edit(
        &self,
        letter_id: &str,
        paper_types: &[PaperType],
    ) -> Result<ApiResource<LetterAttributes>> {
        let types: Vec<&str> = paper_types.iter().map(|pt| pt.as_str()).collect();
        let payload = json!({ "data": { "id": letter_id, "type": "letters",
            "attributes": { "paper_types": types }}});
        let resp = self
            .requestor
            .patch(
                &format!(
                    "/organisations/{}/deliveries/letters/{}",
                    self.org_id, letter_id
                ),
                Some(&payload.to_string()),
            )
            .await?;
        resp.to_resource()
    }

    pub async fn get_file(&self, letter_id: &str) -> Result<Vec<u8>> {
        self.requestor
            .download(&format!(
                "/organisations/{}/deliveries/letters/{}/file",
                self.org_id, letter_id
            ))
            .await
    }

    pub async fn calculate_price(
        &self,
        country: &str,
        paper_types: &[PaperType],
        print_mode: PrintMode,
        print_spectrum: PrintSpectrum,
        delivery_product: DeliveryProduct,
    ) -> Result<ApiResource<LetterPriceAttributes>> {
        let types: Vec<&str> = paper_types.iter().map(|pt| pt.as_str()).collect();
        let payload = json!({ "data": { "type": "letter_price_calculator",
            "attributes": { "country": country, "paper_types": types,
                "print_mode": print_mode.as_str(), "print_spectrum": print_spectrum.as_str(), "delivery_product": delivery_product.as_str() }}});
        let resp = self
            .requestor
            .post(
                &format!(
                    "/organisations/{}/deliveries/letters/price-calculator",
                    self.org_id
                ),
                &payload.to_string(),
            )
            .await?;
        resp.to_resource()
    }
}
