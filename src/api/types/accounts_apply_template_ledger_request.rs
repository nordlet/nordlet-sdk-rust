pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsApplyTemplateLedgerRequest {}

impl AccountsApplyTemplateLedgerRequest {
    pub fn builder() -> AccountsApplyTemplateLedgerRequestBuilder {
        <AccountsApplyTemplateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsApplyTemplateLedgerRequestBuilder {}

impl AccountsApplyTemplateLedgerRequestBuilder {
    /// Consumes the builder and constructs a [`AccountsApplyTemplateLedgerRequest`].
    pub fn build(self) -> Result<AccountsApplyTemplateLedgerRequest, BuildError> {
        Ok(AccountsApplyTemplateLedgerRequest {})
    }
}
