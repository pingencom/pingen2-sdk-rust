use super::requestor::TokenProvider;
use super::*;
use crate::oauth::TokenManager;
use crate::{API_PRODUCTION, API_STAGING};

pub struct PingenClient {
    auth: TokenProvider,
    api_base: String,
}

impl PingenClient {
    pub fn new(access_token: impl Into<TokenProvider>) -> Self {
        Self {
            auth: access_token.into(),
            api_base: API_PRODUCTION.to_string(),
        }
    }

    pub fn new_staging(access_token: impl Into<TokenProvider>) -> Self {
        Self {
            auth: access_token.into(),
            api_base: API_STAGING.to_string(),
        }
    }

    pub fn with_credentials(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        scope: Option<&str>,
    ) -> Self {
        Self::new(TokenManager::client_credentials(
            API_PRODUCTION,
            client_id,
            client_secret,
            scope,
        ))
    }

    pub fn with_credentials_staging(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        scope: Option<&str>,
    ) -> Self {
        Self::new_staging(TokenManager::client_credentials(
            API_STAGING,
            client_id,
            client_secret,
            scope,
        ))
    }

    pub fn letters(&self, org_id: &str) -> Letters {
        Letters::new(org_id, self.auth.clone(), &self.api_base)
    }
    pub fn batches(&self, org_id: &str) -> Batches {
        Batches::new(org_id, self.auth.clone(), &self.api_base)
    }
    pub fn letter_events(&self, org_id: &str) -> LetterEvents {
        LetterEvents::new(org_id, self.auth.clone(), &self.api_base)
    }
    pub fn batch_events(&self, org_id: &str) -> BatchEvents {
        BatchEvents::new(org_id, self.auth.clone(), &self.api_base)
    }
    pub fn organisations(&self) -> Organisations {
        Organisations::new(self.auth.clone(), &self.api_base)
    }
    pub fn users(&self) -> Users {
        Users::new(self.auth.clone(), &self.api_base)
    }
    pub fn user_associations(&self) -> UserAssociations {
        UserAssociations::new(self.auth.clone(), &self.api_base)
    }
    pub fn webhooks(&self, org_id: &str) -> Webhooks {
        Webhooks::new(org_id, self.auth.clone(), &self.api_base)
    }
    pub fn ebills(&self, org_id: &str) -> Ebills {
        Ebills::new(org_id, self.auth.clone(), &self.api_base)
    }
    pub fn ebill_events(&self, org_id: &str) -> EbillEvents {
        EbillEvents::new(org_id, self.auth.clone(), &self.api_base)
    }
    pub fn emails(&self, org_id: &str) -> Emails {
        Emails::new(org_id, self.auth.clone(), &self.api_base)
    }
    pub fn email_events(&self, org_id: &str) -> EmailEvents {
        EmailEvents::new(org_id, self.auth.clone(), &self.api_base)
    }
}
