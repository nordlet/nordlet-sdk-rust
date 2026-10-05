pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JournalTransactionsListLedgerRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<JournalTransactionsListLedgerRequestSortItemDir>,
}

impl JournalTransactionsListLedgerRequestSortItem {
    pub fn builder() -> JournalTransactionsListLedgerRequestSortItemBuilder {
        <JournalTransactionsListLedgerRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JournalTransactionsListLedgerRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<JournalTransactionsListLedgerRequestSortItemDir>,
}

impl JournalTransactionsListLedgerRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: JournalTransactionsListLedgerRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JournalTransactionsListLedgerRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](JournalTransactionsListLedgerRequestSortItemBuilder::field)
    pub fn build(self) -> Result<JournalTransactionsListLedgerRequestSortItem, BuildError> {
        Ok(JournalTransactionsListLedgerRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
