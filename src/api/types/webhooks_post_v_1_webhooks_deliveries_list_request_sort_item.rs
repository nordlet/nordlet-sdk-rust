pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksDeliveriesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1WebhooksDeliveriesListRequestSortItemDir>,
}

impl PostV1WebhooksDeliveriesListRequestSortItem {
    pub fn builder() -> PostV1WebhooksDeliveriesListRequestSortItemBuilder {
        <PostV1WebhooksDeliveriesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksDeliveriesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1WebhooksDeliveriesListRequestSortItemDir>,
}

impl PostV1WebhooksDeliveriesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1WebhooksDeliveriesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksDeliveriesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1WebhooksDeliveriesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1WebhooksDeliveriesListRequestSortItem, BuildError> {
        Ok(PostV1WebhooksDeliveriesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
