pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlatRateConfirmBillingRequest {
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

impl FlatRateConfirmBillingRequest {
    pub fn builder() -> FlatRateConfirmBillingRequestBuilder {
        <FlatRateConfirmBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlatRateConfirmBillingRequestBuilder {
    session_id: Option<String>,
}

impl FlatRateConfirmBillingRequestBuilder {
    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FlatRateConfirmBillingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_id`](FlatRateConfirmBillingRequestBuilder::session_id)
    pub fn build(self) -> Result<FlatRateConfirmBillingRequest, BuildError> {
        Ok(FlatRateConfirmBillingRequest {
            session_id: self
                .session_id
                .ok_or_else(|| BuildError::missing_field("session_id"))?,
        })
    }
}
