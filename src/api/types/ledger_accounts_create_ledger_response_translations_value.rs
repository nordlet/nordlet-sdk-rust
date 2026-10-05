pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsCreateLedgerResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl AccountsCreateLedgerResponseTranslationsValue {
    pub fn builder() -> AccountsCreateLedgerResponseTranslationsValueBuilder {
        <AccountsCreateLedgerResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsCreateLedgerResponseTranslationsValueBuilder {
    name: Option<String>,
}

impl AccountsCreateLedgerResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountsCreateLedgerResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AccountsCreateLedgerResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<AccountsCreateLedgerResponseTranslationsValue, BuildError> {
        Ok(AccountsCreateLedgerResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
