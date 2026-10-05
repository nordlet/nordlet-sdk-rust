pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsListLedgerResponseRowsItemTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl AccountsListLedgerResponseRowsItemTranslationsValue {
    pub fn builder() -> AccountsListLedgerResponseRowsItemTranslationsValueBuilder {
        <AccountsListLedgerResponseRowsItemTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsListLedgerResponseRowsItemTranslationsValueBuilder {
    name: Option<String>,
}

impl AccountsListLedgerResponseRowsItemTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountsListLedgerResponseRowsItemTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AccountsListLedgerResponseRowsItemTranslationsValueBuilder::name)
    pub fn build(self) -> Result<AccountsListLedgerResponseRowsItemTranslationsValue, BuildError> {
        Ok(AccountsListLedgerResponseRowsItemTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
