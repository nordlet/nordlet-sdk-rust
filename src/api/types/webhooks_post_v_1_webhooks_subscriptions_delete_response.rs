pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksSubscriptionsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1WebhooksSubscriptionsDeleteResponse {
    pub fn builder() -> PostV1WebhooksSubscriptionsDeleteResponseBuilder {
        <PostV1WebhooksSubscriptionsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksSubscriptionsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1WebhooksSubscriptionsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksSubscriptionsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1WebhooksSubscriptionsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1WebhooksSubscriptionsDeleteResponse, BuildError> {
        Ok(PostV1WebhooksSubscriptionsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
