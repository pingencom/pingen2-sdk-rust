use crate::api::requestor::ApiRequestor;
use crate::error::{PingenError, Result};
use std::path::Path;

#[derive(Debug)]
pub struct FileUpload<'a> {
    requestor: &'a ApiRequestor,
}

impl<'a> FileUpload<'a> {
    pub fn new(requestor: &'a ApiRequestor) -> Self {
        Self { requestor }
    }

    pub async fn request_file_upload(&self) -> Result<(String, String)> {
        let resp = self.requestor.get("/file-upload", None).await?;
        let data = resp.data.as_ref().ok_or_else(|| PingenError::Api {
            status: 0,
            body: "No data in file-upload response".to_string(),
        })?;
        let url = data["data"]["attributes"]["url"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let sig = data["data"]["attributes"]["url_signature"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        Ok((url, sig))
    }

    pub async fn put_file(&self, file_path: &Path, file_url: &str) -> Result<()> {
        self.requestor.put_file(file_url, file_path).await
    }
}
