pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PublicIntegrationRequestsResponse {
    #[serde(default)]
    pub received: bool,
}

impl PostV1PublicIntegrationRequestsResponse {
    pub fn builder() -> PostV1PublicIntegrationRequestsResponseBuilder {
        <PostV1PublicIntegrationRequestsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PublicIntegrationRequestsResponseBuilder {
    received: Option<bool>,
}

impl PostV1PublicIntegrationRequestsResponseBuilder {
    pub fn received(mut self, value: bool) -> Self {
        self.received = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PublicIntegrationRequestsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`received`](PostV1PublicIntegrationRequestsResponseBuilder::received)
    pub fn build(self) -> Result<PostV1PublicIntegrationRequestsResponse, BuildError> {
        Ok(PostV1PublicIntegrationRequestsResponse {
            received: self
                .received
                .ok_or_else(|| BuildError::missing_field("received"))?,
        })
    }
}
