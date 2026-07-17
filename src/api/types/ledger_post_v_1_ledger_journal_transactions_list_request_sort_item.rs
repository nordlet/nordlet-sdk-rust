pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerJournalTransactionsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1LedgerJournalTransactionsListRequestSortItemDir>,
}

impl PostV1LedgerJournalTransactionsListRequestSortItem {
    pub fn builder() -> PostV1LedgerJournalTransactionsListRequestSortItemBuilder {
        <PostV1LedgerJournalTransactionsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerJournalTransactionsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1LedgerJournalTransactionsListRequestSortItemDir>,
}

impl PostV1LedgerJournalTransactionsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1LedgerJournalTransactionsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerJournalTransactionsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerJournalTransactionsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1LedgerJournalTransactionsListRequestSortItem, BuildError> {
        Ok(PostV1LedgerJournalTransactionsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
