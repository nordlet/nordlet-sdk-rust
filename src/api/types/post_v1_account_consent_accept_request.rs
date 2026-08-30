pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountConsentAcceptRequest {
    #[serde(rename = "acceptTerms")]
    #[serde(default)]
    pub accept_terms: bool,
    #[serde(rename = "acceptDpa")]
    #[serde(default)]
    pub accept_dpa: bool,
}

impl PostV1AccountConsentAcceptRequest {
    pub fn builder() -> PostV1AccountConsentAcceptRequestBuilder {
        <PostV1AccountConsentAcceptRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountConsentAcceptRequestBuilder {
    accept_terms: Option<bool>,
    accept_dpa: Option<bool>,
}

impl PostV1AccountConsentAcceptRequestBuilder {
    pub fn accept_terms(mut self, value: bool) -> Self {
        self.accept_terms = Some(value);
        self
    }

    pub fn accept_dpa(mut self, value: bool) -> Self {
        self.accept_dpa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountConsentAcceptRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accept_terms`](PostV1AccountConsentAcceptRequestBuilder::accept_terms)
    /// - [`accept_dpa`](PostV1AccountConsentAcceptRequestBuilder::accept_dpa)
    pub fn build(self) -> Result<PostV1AccountConsentAcceptRequest, BuildError> {
        Ok(PostV1AccountConsentAcceptRequest {
            accept_terms: self
                .accept_terms
                .ok_or_else(|| BuildError::missing_field("accept_terms"))?,
            accept_dpa: self
                .accept_dpa
                .ok_or_else(|| BuildError::missing_field("accept_dpa"))?,
        })
    }
}
