pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TopupConfirmBillingRequest {
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

impl TopupConfirmBillingRequest {
    pub fn builder() -> TopupConfirmBillingRequestBuilder {
        <TopupConfirmBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopupConfirmBillingRequestBuilder {
    session_id: Option<String>,
}

impl TopupConfirmBillingRequestBuilder {
    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TopupConfirmBillingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_id`](TopupConfirmBillingRequestBuilder::session_id)
    pub fn build(self) -> Result<TopupConfirmBillingRequest, BuildError> {
        Ok(TopupConfirmBillingRequest {
            session_id: self
                .session_id
                .ok_or_else(|| BuildError::missing_field("session_id"))?,
        })
    }
}
