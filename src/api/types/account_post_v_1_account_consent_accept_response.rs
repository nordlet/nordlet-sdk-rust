pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountConsentAcceptResponse {
    #[serde(rename = "termsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_version: Option<String>,
    #[serde(rename = "termsAcceptedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_accepted_at: Option<String>,
    #[serde(rename = "dpaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpa_version: Option<String>,
    #[serde(rename = "dpaAcceptedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpa_accepted_at: Option<String>,
    #[serde(rename = "currentTermsVersion")]
    #[serde(default)]
    pub current_terms_version: String,
    #[serde(rename = "currentDpaVersion")]
    #[serde(default)]
    pub current_dpa_version: String,
    #[serde(default)]
    pub required: bool,
}

impl PostV1AccountConsentAcceptResponse {
    pub fn builder() -> PostV1AccountConsentAcceptResponseBuilder {
        <PostV1AccountConsentAcceptResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountConsentAcceptResponseBuilder {
    terms_version: Option<String>,
    terms_accepted_at: Option<String>,
    dpa_version: Option<String>,
    dpa_accepted_at: Option<String>,
    current_terms_version: Option<String>,
    current_dpa_version: Option<String>,
    required: Option<bool>,
}

impl PostV1AccountConsentAcceptResponseBuilder {
    pub fn terms_version(mut self, value: impl Into<String>) -> Self {
        self.terms_version = Some(value.into());
        self
    }

    pub fn terms_accepted_at(mut self, value: impl Into<String>) -> Self {
        self.terms_accepted_at = Some(value.into());
        self
    }

    pub fn dpa_version(mut self, value: impl Into<String>) -> Self {
        self.dpa_version = Some(value.into());
        self
    }

    pub fn dpa_accepted_at(mut self, value: impl Into<String>) -> Self {
        self.dpa_accepted_at = Some(value.into());
        self
    }

    pub fn current_terms_version(mut self, value: impl Into<String>) -> Self {
        self.current_terms_version = Some(value.into());
        self
    }

    pub fn current_dpa_version(mut self, value: impl Into<String>) -> Self {
        self.current_dpa_version = Some(value.into());
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountConsentAcceptResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`current_terms_version`](PostV1AccountConsentAcceptResponseBuilder::current_terms_version)
    /// - [`current_dpa_version`](PostV1AccountConsentAcceptResponseBuilder::current_dpa_version)
    /// - [`required`](PostV1AccountConsentAcceptResponseBuilder::required)
    pub fn build(self) -> Result<PostV1AccountConsentAcceptResponse, BuildError> {
        Ok(PostV1AccountConsentAcceptResponse {
            terms_version: self.terms_version,
            terms_accepted_at: self.terms_accepted_at,
            dpa_version: self.dpa_version,
            dpa_accepted_at: self.dpa_accepted_at,
            current_terms_version: self
                .current_terms_version
                .ok_or_else(|| BuildError::missing_field("current_terms_version"))?,
            current_dpa_version: self
                .current_dpa_version
                .ok_or_else(|| BuildError::missing_field("current_dpa_version"))?,
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
        })
    }
}
