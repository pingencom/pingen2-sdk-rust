use crate::error::{PingenError, Result};
use crate::{AUTH_PRODUCTION, AUTH_STAGING};
use reqwest::Url;
use serde_json::Value;
use std::collections::HashMap;

pub struct OAuth;

impl OAuth {
    fn set_client_id(
        params: &mut HashMap<String, String>,
        default_client_id: Option<&str>,
    ) -> Result<()> {
        if params.contains_key("client_id") {
            return Ok(());
        }

        if let Some(id) = default_client_id {
            params.insert("client_id".to_string(), id.to_string());
            return Ok(());
        }

        Err(PingenError::Authentication(
            "No client_id provided. (HINT: pass client_id in params or as default_client_id)"
                .to_string(),
        ))
    }

    fn set_client_secret(
        params: &mut HashMap<String, String>,
        default_client_secret: Option<&str>,
    ) -> Result<()> {
        if params.contains_key("client_secret") {
            return Ok(());
        }

        if let Some(secret) = default_client_secret {
            params.insert("client_secret".to_string(), secret.to_string());
            return Ok(());
        }

        Err(PingenError::Authentication(
            "No client_secret provided. (HINT: pass client_secret in params or as default_client_secret)".to_string(),
        ))
    }

    pub fn authorize_url(
        use_staging: bool,
        default_client_id: Option<&str>,
        mut params: HashMap<String, String>,
    ) -> Result<String> {
        let base = if use_staging {
            AUTH_STAGING
        } else {
            AUTH_PRODUCTION
        };

        Self::set_client_id(&mut params, default_client_id)?;

        params
            .entry("response_type".to_string())
            .or_insert_with(|| "code".to_string());

        let mut url = Url::parse(base)
            .map_err(|e| PingenError::Authentication(format!("Invalid base URL: {}", e)))?;
        {
            let mut query = url.query_pairs_mut();
            for (k, v) in &params {
                query.append_pair(k, v);
            }
        }

        Ok(url.to_string())
    }

    pub async fn get_token(
        api_base: &str,
        default_client_id: Option<&str>,
        default_client_secret: Option<&str>,
        mut params: HashMap<String, String>,
    ) -> Result<Value> {
        Self::set_client_id(&mut params, default_client_id)?;
        Self::set_client_secret(&mut params, default_client_secret)?;

        let pairs: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let url = format!("{}/auth/access-tokens", api_base);
        Self::post_form(&url, &pairs).await
    }

    pub async fn post_form(url: &str, params: &[(&str, &str)]) -> Result<Value> {
        let client = reqwest::Client::new();
        let resp = client.post(url).form(params).send().await?;
        let status = resp.status().as_u16();
        let text = resp.text().await?;
        if (200..310).contains(&status) {
            Ok(serde_json::from_str(&text)?)
        } else {
            Err(PingenError::Api { status, body: text })
        }
    }

    pub fn get_token_from_implicit(fragment: &str) -> HashMap<String, String> {
        fragment
            .split('&')
            .filter_map(|pair| {
                pair.split_once('=').map(|(k, v)| {
                    (
                        urlencoding::decode(k)
                            .map(|c| c.into_owned())
                            .unwrap_or_else(|_| k.to_string()),
                        urlencoding::decode(v)
                            .map(|c| c.into_owned())
                            .unwrap_or_else(|_| v.to_string()),
                    )
                })
            })
            .collect()
    }
}

/// Caches an OAuth access token in memory and reuses it across calls as long as it
/// has not expired, only requesting a new one from the token endpoint when needed.
///
/// Combined with [`crate::api::requestor::TokenProvider::Managed`] this gives every
/// API struct transparent token auto-refresh: each request asks the manager for a
/// token and an expired one is replaced automatically before the request is sent.
pub struct TokenManager {
    api_base: String,
    default_client_id: Option<String>,
    default_client_secret: Option<String>,
    default_params: HashMap<String, String>,
    cached: tokio::sync::Mutex<Option<CachedToken>>,
}

// Hand-written so the client secret and any cached bearer token are never
// surfaced through `{:?}` (a divergence from a derived Debug that would leak
// credentials into logs).
impl std::fmt::Debug for TokenManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenManager")
            .field("api_base", &self.api_base)
            .field("default_client_id", &self.default_client_id)
            .field(
                "default_client_secret",
                &self.default_client_secret.as_ref().map(|_| "[redacted]"),
            )
            .field("default_params", &self.default_params)
            .field("cached", &"[redacted]")
            .finish()
    }
}

#[derive(Clone)]
struct CachedToken {
    access_token: String,
    expires_at: std::time::SystemTime,
}

const TOKEN_EXPIRY_SAFETY_MARGIN: std::time::Duration = std::time::Duration::from_secs(300);

impl TokenManager {
    pub fn new(
        api_base: impl Into<String>,
        default_client_id: Option<String>,
        default_client_secret: Option<String>,
    ) -> Self {
        let mut default_params = HashMap::new();
        default_params.insert("grant_type".to_string(), "client_credentials".to_string());
        Self {
            api_base: api_base.into(),
            default_client_id,
            default_client_secret,
            default_params,
            cached: tokio::sync::Mutex::new(None),
        }
    }

    pub fn client_credentials(
        api_base: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        scope: Option<&str>,
    ) -> Self {
        let mut manager = Self::new(api_base, Some(client_id.into()), Some(client_secret.into()));
        if let Some(scope) = scope {
            manager
                .default_params
                .insert("scope".to_string(), scope.to_string());
        }
        manager
    }

    pub async fn get_access_token(&self) -> Result<String> {
        {
            let cache = self.cached.lock().await;
            if let Some(token) = cache.as_ref() {
                if token.expires_at > std::time::SystemTime::now() {
                    return Ok(token.access_token.clone());
                }
            }
        }

        let value = OAuth::get_token(
            &self.api_base,
            self.default_client_id.as_deref(),
            self.default_client_secret.as_deref(),
            self.default_params.clone(),
        )
        .await?;
        let token: crate::dto::TokenResponse = serde_json::from_value(value)?;
        let ttl = std::time::Duration::from_secs(token.expires_in)
            .saturating_sub(TOKEN_EXPIRY_SAFETY_MARGIN);
        let expires_at = std::time::SystemTime::now()
            .checked_add(ttl)
            .unwrap_or_else(std::time::SystemTime::now);
        let access_token = token.access_token.clone();
        *self.cached.lock().await = Some(CachedToken {
            access_token: access_token.clone(),
            expires_at,
        });
        Ok(access_token)
    }

    pub async fn invalidate(&self) {
        *self.cached.lock().await = None;
    }
}
