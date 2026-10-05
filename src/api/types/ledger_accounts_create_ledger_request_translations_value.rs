pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsCreateLedgerRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl AccountsCreateLedgerRequestTranslationsValue {
    pub fn builder() -> AccountsCreateLedgerRequestTranslationsValueBuilder {
        <AccountsCreateLedgerRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsCreateLedgerRequestTranslationsValueBuilder {
    name: Option<String>,
}

impl AccountsCreateLedgerRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountsCreateLedgerRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AccountsCreateLedgerRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<AccountsCreateLedgerRequestTranslationsValue, BuildError> {
        Ok(AccountsCreateLedgerRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
