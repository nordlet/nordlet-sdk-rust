pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsUpdateLedgerRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl AccountsUpdateLedgerRequestTranslationsValue {
    pub fn builder() -> AccountsUpdateLedgerRequestTranslationsValueBuilder {
        <AccountsUpdateLedgerRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsUpdateLedgerRequestTranslationsValueBuilder {
    name: Option<String>,
}

impl AccountsUpdateLedgerRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountsUpdateLedgerRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AccountsUpdateLedgerRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<AccountsUpdateLedgerRequestTranslationsValue, BuildError> {
        Ok(AccountsUpdateLedgerRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
