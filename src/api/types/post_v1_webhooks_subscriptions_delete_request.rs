pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksSubscriptionsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1WebhooksSubscriptionsDeleteRequest {
    pub fn builder() -> PostV1WebhooksSubscriptionsDeleteRequestBuilder {
        <PostV1WebhooksSubscriptionsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksSubscriptionsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1WebhooksSubscriptionsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksSubscriptionsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1WebhooksSubscriptionsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1WebhooksSubscriptionsDeleteRequest, BuildError> {
        Ok(PostV1WebhooksSubscriptionsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
