pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlatRateCreateBillingResponse {
    #[serde(default)]
    pub url: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

impl FlatRateCreateBillingResponse {
    pub fn builder() -> FlatRateCreateBillingResponseBuilder {
        <FlatRateCreateBillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlatRateCreateBillingResponseBuilder {
    url: Option<String>,
    session_id: Option<String>,
}

impl FlatRateCreateBillingResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FlatRateCreateBillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](FlatRateCreateBillingResponseBuilder::url)
    /// - [`session_id`](FlatRateCreateBillingResponseBuilder::session_id)
    pub fn build(self) -> Result<FlatRateCreateBillingResponse, BuildError> {
        Ok(FlatRateCreateBillingResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            session_id: self
                .session_id
                .ok_or_else(|| BuildError::missing_field("session_id"))?,
        })
    }
}
