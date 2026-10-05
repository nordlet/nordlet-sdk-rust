pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConsentAcceptAccountRequest {
    #[serde(rename = "acceptTerms")]
    #[serde(default)]
    pub accept_terms: bool,
    #[serde(rename = "acceptDpa")]
    #[serde(default)]
    pub accept_dpa: bool,
}

impl ConsentAcceptAccountRequest {
    pub fn builder() -> ConsentAcceptAccountRequestBuilder {
        <ConsentAcceptAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsentAcceptAccountRequestBuilder {
    accept_terms: Option<bool>,
    accept_dpa: Option<bool>,
}

impl ConsentAcceptAccountRequestBuilder {
    pub fn accept_terms(mut self, value: bool) -> Self {
        self.accept_terms = Some(value);
        self
    }

    pub fn accept_dpa(mut self, value: bool) -> Self {
        self.accept_dpa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsentAcceptAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accept_terms`](ConsentAcceptAccountRequestBuilder::accept_terms)
    /// - [`accept_dpa`](ConsentAcceptAccountRequestBuilder::accept_dpa)
    pub fn build(self) -> Result<ConsentAcceptAccountRequest, BuildError> {
        Ok(ConsentAcceptAccountRequest {
            accept_terms: self
                .accept_terms
                .ok_or_else(|| BuildError::missing_field("accept_terms"))?,
            accept_dpa: self
                .accept_dpa
                .ok_or_else(|| BuildError::missing_field("accept_dpa"))?,
        })
    }
}
