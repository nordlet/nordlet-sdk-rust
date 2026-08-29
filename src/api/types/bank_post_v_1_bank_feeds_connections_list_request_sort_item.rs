pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsConnectionsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1BankFeedsConnectionsListRequestSortItemDir>,
}

impl PostV1BankFeedsConnectionsListRequestSortItem {
    pub fn builder() -> PostV1BankFeedsConnectionsListRequestSortItemBuilder {
        <PostV1BankFeedsConnectionsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1BankFeedsConnectionsListRequestSortItemDir>,
}

impl PostV1BankFeedsConnectionsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1BankFeedsConnectionsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankFeedsConnectionsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsListRequestSortItem, BuildError> {
        Ok(PostV1BankFeedsConnectionsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
