pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksDeliveriesRedeliverRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1WebhooksDeliveriesRedeliverRequest {
    pub fn builder() -> PostV1WebhooksDeliveriesRedeliverRequestBuilder {
        <PostV1WebhooksDeliveriesRedeliverRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksDeliveriesRedeliverRequestBuilder {
    id: Option<String>,
}

impl PostV1WebhooksDeliveriesRedeliverRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksDeliveriesRedeliverRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1WebhooksDeliveriesRedeliverRequestBuilder::id)
    pub fn build(self) -> Result<PostV1WebhooksDeliveriesRedeliverRequest, BuildError> {
        Ok(PostV1WebhooksDeliveriesRedeliverRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
