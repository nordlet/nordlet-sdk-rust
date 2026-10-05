pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsListLedgerRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<AccountsListLedgerRequestSortItemDir>,
}

impl AccountsListLedgerRequestSortItem {
    pub fn builder() -> AccountsListLedgerRequestSortItemBuilder {
        <AccountsListLedgerRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsListLedgerRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<AccountsListLedgerRequestSortItemDir>,
}

impl AccountsListLedgerRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: AccountsListLedgerRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsListLedgerRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AccountsListLedgerRequestSortItemBuilder::field)
    pub fn build(self) -> Result<AccountsListLedgerRequestSortItem, BuildError> {
        Ok(AccountsListLedgerRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
