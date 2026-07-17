pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1LedgerJournalTransactionsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1LedgerJournalTransactionsListRequestFilterItemOp,
    pub value: PostV1LedgerJournalTransactionsListRequestFilterItemValue,
}

impl PostV1LedgerJournalTransactionsListRequestFilterItem {
    pub fn builder() -> PostV1LedgerJournalTransactionsListRequestFilterItemBuilder {
        <PostV1LedgerJournalTransactionsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerJournalTransactionsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1LedgerJournalTransactionsListRequestFilterItemOp>,
    value: Option<PostV1LedgerJournalTransactionsListRequestFilterItemValue>,
}

impl PostV1LedgerJournalTransactionsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1LedgerJournalTransactionsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(
        mut self,
        value: PostV1LedgerJournalTransactionsListRequestFilterItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerJournalTransactionsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerJournalTransactionsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1LedgerJournalTransactionsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1LedgerJournalTransactionsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1LedgerJournalTransactionsListRequestFilterItem, BuildError> {
        Ok(PostV1LedgerJournalTransactionsListRequestFilterItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
