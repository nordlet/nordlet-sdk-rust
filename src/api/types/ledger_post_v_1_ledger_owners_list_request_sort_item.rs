pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerOwnersListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1LedgerOwnersListRequestSortItemDir>,
}

impl PostV1LedgerOwnersListRequestSortItem {
    pub fn builder() -> PostV1LedgerOwnersListRequestSortItemBuilder {
        <PostV1LedgerOwnersListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerOwnersListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1LedgerOwnersListRequestSortItemDir>,
}

impl PostV1LedgerOwnersListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1LedgerOwnersListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerOwnersListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerOwnersListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1LedgerOwnersListRequestSortItem, BuildError> {
        Ok(PostV1LedgerOwnersListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
