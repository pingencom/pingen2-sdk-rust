use super::*;
use crate::{API_PRODUCTION, API_STAGING};

pub struct PingenClient {
    access_token: String,
    api_base: String,
}

impl PingenClient {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            api_base: API_PRODUCTION.to_string(),
        }
    }

    pub fn new_staging(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            api_base: API_STAGING.to_string(),
        }
    }

    pub fn letters(&self, org_id: &str) -> Letters {
        Letters::new(org_id, &self.access_token, &self.api_base)
    }
    pub fn batches(&self, org_id: &str) -> Batches {
        Batches::new(org_id, &self.access_token, &self.api_base)
    }
    pub fn letter_events(&self, org_id: &str) -> LetterEvents {
        LetterEvents::new(org_id, &self.access_token, &self.api_base)
    }
    pub fn batch_events(&self, org_id: &str) -> BatchEvents {
        BatchEvents::new(org_id, &self.access_token, &self.api_base)
    }
    pub fn organisations(&self) -> Organisations {
        Organisations::new(&self.access_token, &self.api_base)
    }
    pub fn users(&self) -> Users {
        Users::new(&self.access_token, &self.api_base)
    }
    pub fn user_associations(&self) -> UserAssociations {
        UserAssociations::new(&self.access_token, &self.api_base)
    }
    pub fn webhooks(&self, org_id: &str) -> Webhooks {
        Webhooks::new(org_id, &self.access_token, &self.api_base)
    }
    pub fn ebills(&self, org_id: &str) -> Ebills {
        Ebills::new(org_id, &self.access_token, &self.api_base)
    }
    pub fn emails(&self, org_id: &str) -> Emails {
        Emails::new(org_id, &self.access_token, &self.api_base)
    }
}
