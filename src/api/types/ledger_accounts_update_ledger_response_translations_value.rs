pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsUpdateLedgerResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl AccountsUpdateLedgerResponseTranslationsValue {
    pub fn builder() -> AccountsUpdateLedgerResponseTranslationsValueBuilder {
        <AccountsUpdateLedgerResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsUpdateLedgerResponseTranslationsValueBuilder {
    name: Option<String>,
}

impl AccountsUpdateLedgerResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountsUpdateLedgerResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AccountsUpdateLedgerResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<AccountsUpdateLedgerResponseTranslationsValue, BuildError> {
        Ok(AccountsUpdateLedgerResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
