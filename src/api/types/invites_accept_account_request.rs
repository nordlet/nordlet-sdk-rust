pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesAcceptAccountRequest {
    #[serde(default)]
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<InvitesAcceptAccountRequestLocale>,
    #[serde(rename = "acceptTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_terms: Option<bool>,
    #[serde(rename = "acceptDpa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_dpa: Option<bool>,
}

impl InvitesAcceptAccountRequest {
    pub fn builder() -> InvitesAcceptAccountRequestBuilder {
        <InvitesAcceptAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesAcceptAccountRequestBuilder {
    token: Option<String>,
    name: Option<String>,
    locale: Option<InvitesAcceptAccountRequestLocale>,
    accept_terms: Option<bool>,
    accept_dpa: Option<bool>,
}

impl InvitesAcceptAccountRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn locale(mut self, value: InvitesAcceptAccountRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn accept_terms(mut self, value: bool) -> Self {
        self.accept_terms = Some(value);
        self
    }

    pub fn accept_dpa(mut self, value: bool) -> Self {
        self.accept_dpa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitesAcceptAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](InvitesAcceptAccountRequestBuilder::token)
    pub fn build(self) -> Result<InvitesAcceptAccountRequest, BuildError> {
        Ok(InvitesAcceptAccountRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            name: self.name,
            locale: self.locale,
            accept_terms: self.accept_terms,
            accept_dpa: self.accept_dpa,
        })
    }
}
