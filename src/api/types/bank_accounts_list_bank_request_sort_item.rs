pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsListBankRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<AccountsListBankRequestSortItemDir>,
}

impl AccountsListBankRequestSortItem {
    pub fn builder() -> AccountsListBankRequestSortItemBuilder {
        <AccountsListBankRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsListBankRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<AccountsListBankRequestSortItemDir>,
}

impl AccountsListBankRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: AccountsListBankRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsListBankRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AccountsListBankRequestSortItemBuilder::field)
    pub fn build(self) -> Result<AccountsListBankRequestSortItem, BuildError> {
        Ok(AccountsListBankRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
