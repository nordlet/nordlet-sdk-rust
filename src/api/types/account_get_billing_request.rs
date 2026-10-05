pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountGetBillingRequest {}

impl AccountGetBillingRequest {
    pub fn builder() -> AccountGetBillingRequestBuilder {
        <AccountGetBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountGetBillingRequestBuilder {}

impl AccountGetBillingRequestBuilder {
    /// Consumes the builder and constructs a [`AccountGetBillingRequest`].
    pub fn build(self) -> Result<AccountGetBillingRequest, BuildError> {
        Ok(AccountGetBillingRequest {})
    }
}
