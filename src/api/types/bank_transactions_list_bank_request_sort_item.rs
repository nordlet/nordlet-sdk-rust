pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsListBankRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<TransactionsListBankRequestSortItemDir>,
}

impl TransactionsListBankRequestSortItem {
    pub fn builder() -> TransactionsListBankRequestSortItemBuilder {
        <TransactionsListBankRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsListBankRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<TransactionsListBankRequestSortItemDir>,
}

impl TransactionsListBankRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: TransactionsListBankRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsListBankRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](TransactionsListBankRequestSortItemBuilder::field)
    pub fn build(self) -> Result<TransactionsListBankRequestSortItem, BuildError> {
        Ok(TransactionsListBankRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
