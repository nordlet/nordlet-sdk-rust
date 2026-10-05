pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntegrationRequestsPublicResponse {
    #[serde(default)]
    pub received: bool,
}

impl IntegrationRequestsPublicResponse {
    pub fn builder() -> IntegrationRequestsPublicResponseBuilder {
        <IntegrationRequestsPublicResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntegrationRequestsPublicResponseBuilder {
    received: Option<bool>,
}

impl IntegrationRequestsPublicResponseBuilder {
    pub fn received(mut self, value: bool) -> Self {
        self.received = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntegrationRequestsPublicResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`received`](IntegrationRequestsPublicResponseBuilder::received)
    pub fn build(self) -> Result<IntegrationRequestsPublicResponse, BuildError> {
        Ok(IntegrationRequestsPublicResponse {
            received: self
                .received
                .ok_or_else(|| BuildError::missing_field("received"))?,
        })
    }
}
