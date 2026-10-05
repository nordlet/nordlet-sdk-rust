pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TopupCreateBillingResponse {
    #[serde(default)]
    pub url: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

impl TopupCreateBillingResponse {
    pub fn builder() -> TopupCreateBillingResponseBuilder {
        <TopupCreateBillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopupCreateBillingResponseBuilder {
    url: Option<String>,
    session_id: Option<String>,
}

impl TopupCreateBillingResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TopupCreateBillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](TopupCreateBillingResponseBuilder::url)
    /// - [`session_id`](TopupCreateBillingResponseBuilder::session_id)
    pub fn build(self) -> Result<TopupCreateBillingResponse, BuildError> {
        Ok(TopupCreateBillingResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            session_id: self
                .session_id
                .ok_or_else(|| BuildError::missing_field("session_id"))?,
        })
    }
}
