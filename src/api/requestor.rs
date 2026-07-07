use crate::error::{PingenError, Result};
use crate::response::PingenResponse;
use reqwest::Client;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use url::Url;

const USER_AGENT: &str = "PINGEN.SDK.RUST";

#[derive(Debug, Clone)]
pub struct ApiRequestor {
    access_token: String,
    pub(crate) api_base: String,
    client: Client,
}

impl ApiRequestor {
    pub fn new(access_token: impl Into<String>, api_base: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .expect("Failed to build reqwest HTTP client");
        Self {
            access_token: access_token.into(),
            api_base: api_base.into(),
            client,
        }
    }

    fn default_headers(&self) -> reqwest::header::HeaderMap {
        use reqwest::header;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            USER_AGENT
                .parse()
                .expect("static User-Agent header is valid"),
        );
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", self.access_token)
                .parse()
                .expect("Bearer token header should be valid ASCII"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            "application/vnd.api+json"
                .parse()
                .expect("static Content-Type header is valid"),
        );
        headers.insert(
            header::ACCEPT,
            "application/vnd.api+json"
                .parse()
                .expect("static Accept header is valid"),
        );
        headers
    }

    fn build_url(&self, path: &str, params: &HashMap<String, String>) -> Result<String> {
        let raw = format!("{}{}", self.api_base, path);
        if params.is_empty() {
            return Ok(raw);
        }
        let mut url = Url::parse(&raw).map_err(|e| PingenError::Api {
            status: 0,
            body: format!("Invalid URL: {e}"),
        })?;
        for (k, v) in params {
            url.query_pairs_mut().append_pair(k, v);
        }
        Ok(url.to_string())
    }

    async fn interpret(&self, response: reqwest::Response) -> Result<PingenResponse> {
        let status = response.status().as_u16();
        let mut headers = HashMap::new();
        for (k, v) in response.headers() {
            if let Ok(val) = v.to_str() {
                headers.insert(k.to_string(), val.to_string());
            }
        }
        let body = response.text().await?;
        if (200..310).contains(&status) {
            Ok(PingenResponse::new(body, status, headers))
        } else {
            Err(PingenError::Api { status, body })
        }
    }

    pub async fn get(
        &self,
        path: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<PingenResponse> {
        let empty = HashMap::new();
        let url = self.build_url(path, params.unwrap_or(&empty))?;
        let resp = self
            .client
            .get(&url)
            .headers(self.default_headers())
            .send()
            .await?;
        self.interpret(resp).await
    }

    pub async fn post(&self, path: &str, payload: &str) -> Result<PingenResponse> {
        let url = format!("{}{}", self.api_base, path);
        let resp = self
            .client
            .post(&url)
            .headers(self.default_headers())
            .body(payload.to_string())
            .send()
            .await?;
        self.interpret(resp).await
    }

    pub async fn patch(&self, path: &str, payload: Option<&str>) -> Result<PingenResponse> {
        let url = format!("{}{}", self.api_base, path);
        let mut req = self.client.patch(&url).headers(self.default_headers());
        if let Some(body) = payload {
            req = req.body(body.to_string());
        }
        self.interpret(req.send().await?).await
    }

    pub async fn delete(&self, path: &str) -> Result<PingenResponse> {
        let url = format!("{}{}", self.api_base, path);
        let resp = self
            .client
            .delete(&url)
            .headers(self.default_headers())
            .send()
            .await?;
        self.interpret(resp).await
    }

    pub async fn put_file(&self, url: &str, file_path: &Path) -> Result<()> {
        let bytes = tokio::fs::read(file_path).await?;
        let client = Client::builder().timeout(Duration::from_secs(60)).build()?;
        let resp = client.put(url).body(bytes).send().await?;
        let status = resp.status().as_u16();
        if (200..310).contains(&status) {
            Ok(())
        } else {
            let body = resp.text().await.unwrap_or_default();
            Err(PingenError::Api { status, body })
        }
    }

    /// Downloads binary content (e.g. a rendered PDF) as raw bytes, unlike `get`/`interpret`
    /// which decode the response as UTF-8 text and would corrupt binary payloads.
    pub async fn download(&self, path: &str) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.api_base, path);
        let resp = self
            .client
            .get(&url)
            .headers(self.default_headers())
            .send()
            .await?;
        let status = resp.status().as_u16();
        if (200..310).contains(&status) {
            Ok(resp.bytes().await?.to_vec())
        } else {
            let body = resp.text().await.unwrap_or_default();
            Err(PingenError::Api { status, body })
        }
    }
}
