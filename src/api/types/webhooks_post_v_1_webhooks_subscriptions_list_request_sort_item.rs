pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksSubscriptionsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1WebhooksSubscriptionsListRequestSortItemDir>,
}

impl PostV1WebhooksSubscriptionsListRequestSortItem {
    pub fn builder() -> PostV1WebhooksSubscriptionsListRequestSortItemBuilder {
        <PostV1WebhooksSubscriptionsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksSubscriptionsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1WebhooksSubscriptionsListRequestSortItemDir>,
}

impl PostV1WebhooksSubscriptionsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1WebhooksSubscriptionsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksSubscriptionsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1WebhooksSubscriptionsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1WebhooksSubscriptionsListRequestSortItem, BuildError> {
        Ok(PostV1WebhooksSubscriptionsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
