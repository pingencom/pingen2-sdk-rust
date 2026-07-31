use crate::error::{PingenError, Result};
use crate::oauth::TokenManager;
use crate::response::PingenResponse;
use reqwest::Client;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

const USER_AGENT: &str = "PINGEN.SDK.RUST";

/// Where the requestor gets its bearer token from: either a fixed token string or
/// a shared [`TokenManager`] that transparently refreshes expired tokens.
#[derive(Clone)]
pub enum TokenProvider {
    Static(String),
    Managed(Arc<TokenManager>),
}

// Hand-written so a static bearer token is never surfaced through `{:?}`; the
// managed variant delegates to `TokenManager`'s own redacting Debug. This also
// keeps the derived `Debug` on `ApiRequestor` (which embeds a `TokenProvider`)
// leak-free.
impl std::fmt::Debug for TokenProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static(_) => f.debug_tuple("Static").field(&"[redacted]").finish(),
            Self::Managed(manager) => f.debug_tuple("Managed").field(manager).finish(),
        }
    }
}

impl TokenProvider {
    pub async fn access_token(&self) -> Result<String> {
        match self {
            Self::Static(token) => Ok(token.clone()),
            Self::Managed(manager) => manager.get_access_token().await,
        }
    }
}

impl From<String> for TokenProvider {
    fn from(token: String) -> Self {
        Self::Static(token)
    }
}

impl From<&str> for TokenProvider {
    fn from(token: &str) -> Self {
        Self::Static(token.to_string())
    }
}

impl From<&String> for TokenProvider {
    fn from(token: &String) -> Self {
        Self::Static(token.clone())
    }
}

impl From<TokenManager> for TokenProvider {
    fn from(manager: TokenManager) -> Self {
        Self::Managed(Arc::new(manager))
    }
}

impl From<Arc<TokenManager>> for TokenProvider {
    fn from(manager: Arc<TokenManager>) -> Self {
        Self::Managed(manager)
    }
}

impl From<&Arc<TokenManager>> for TokenProvider {
    fn from(manager: &Arc<TokenManager>) -> Self {
        Self::Managed(Arc::clone(manager))
    }
}

#[derive(Debug, Clone)]
pub struct ApiRequestor {
    token_provider: TokenProvider,
    pub(crate) api_base: String,
    client: Client,
}

impl ApiRequestor {
    pub fn new(token_provider: impl Into<TokenProvider>, api_base: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .expect("Failed to build reqwest HTTP client");
        Self {
            token_provider: token_provider.into(),
            api_base: api_base.into(),
            client,
        }
    }

    async fn default_headers(&self) -> Result<reqwest::header::HeaderMap> {
        use reqwest::header;

        let access_token = self.token_provider.access_token().await?;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            USER_AGENT
                .parse()
                .expect("static User-Agent header is valid"),
        );
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", access_token).parse().map_err(|_| {
                PingenError::Authentication(
                    "Access token contains characters that are invalid in an Authorization header"
                        .to_string(),
                )
            })?,
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
        Ok(headers)
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
            .headers(self.default_headers().await?)
            .send()
            .await?;
        self.interpret(resp).await
    }

    pub async fn post(&self, path: &str, payload: &str) -> Result<PingenResponse> {
        let url = format!("{}{}", self.api_base, path);
        let resp = self
            .client
            .post(&url)
            .headers(self.default_headers().await?)
            .body(payload.to_string())
            .send()
            .await?;
        self.interpret(resp).await
    }

    pub async fn patch(&self, path: &str, payload: Option<&str>) -> Result<PingenResponse> {
        let url = format!("{}{}", self.api_base, path);
        let mut req = self
            .client
            .patch(&url)
            .headers(self.default_headers().await?);
        if let Some(body) = payload {
            req = req.body(body.to_string());
        }
        self.interpret(req.send().await?).await
    }

    pub async fn delete(&self, path: &str, payload: Option<&str>) -> Result<PingenResponse> {
        let url = format!("{}{}", self.api_base, path);
        let mut req = self
            .client
            .delete(&url)
            .headers(self.default_headers().await?);
        if let Some(body) = payload {
            req = req.body(body.to_string());
        }
        self.interpret(req.send().await?).await
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
            .headers(self.default_headers().await?)
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
