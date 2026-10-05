pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsApplyTemplateLedgerResponse {
    #[serde(default)]
    pub accounts: i64,
}

impl AccountsApplyTemplateLedgerResponse {
    pub fn builder() -> AccountsApplyTemplateLedgerResponseBuilder {
        <AccountsApplyTemplateLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsApplyTemplateLedgerResponseBuilder {
    accounts: Option<i64>,
}

impl AccountsApplyTemplateLedgerResponseBuilder {
    pub fn accounts(mut self, value: i64) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsApplyTemplateLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accounts`](AccountsApplyTemplateLedgerResponseBuilder::accounts)
    pub fn build(self) -> Result<AccountsApplyTemplateLedgerResponse, BuildError> {
        Ok(AccountsApplyTemplateLedgerResponse {
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
        })
    }
}
