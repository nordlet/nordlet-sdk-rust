pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1LedgerAccountsListRequestSortItemDir>,
}

impl PostV1LedgerAccountsListRequestSortItem {
    pub fn builder() -> PostV1LedgerAccountsListRequestSortItemBuilder {
        <PostV1LedgerAccountsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1LedgerAccountsListRequestSortItemDir>,
}

impl PostV1LedgerAccountsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1LedgerAccountsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerAccountsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1LedgerAccountsListRequestSortItem, BuildError> {
        Ok(PostV1LedgerAccountsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
