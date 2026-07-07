use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FileUploadAttributes {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub url_signature: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
}
