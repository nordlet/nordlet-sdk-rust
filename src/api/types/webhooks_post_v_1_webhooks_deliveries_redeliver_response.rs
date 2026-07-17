pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksDeliveriesRedeliverResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
}

impl PostV1WebhooksDeliveriesRedeliverResponse {
    pub fn builder() -> PostV1WebhooksDeliveriesRedeliverResponseBuilder {
        <PostV1WebhooksDeliveriesRedeliverResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksDeliveriesRedeliverResponseBuilder {
    id: Option<String>,
    status: Option<String>,
}

impl PostV1WebhooksDeliveriesRedeliverResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksDeliveriesRedeliverResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1WebhooksDeliveriesRedeliverResponseBuilder::id)
    /// - [`status`](PostV1WebhooksDeliveriesRedeliverResponseBuilder::status)
    pub fn build(self) -> Result<PostV1WebhooksDeliveriesRedeliverResponse, BuildError> {
        Ok(PostV1WebhooksDeliveriesRedeliverResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
