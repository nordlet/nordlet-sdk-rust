pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsSwitchChartLedgerRequest {}

impl AccountsSwitchChartLedgerRequest {
    pub fn builder() -> AccountsSwitchChartLedgerRequestBuilder {
        <AccountsSwitchChartLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsSwitchChartLedgerRequestBuilder {}

impl AccountsSwitchChartLedgerRequestBuilder {
    /// Consumes the builder and constructs a [`AccountsSwitchChartLedgerRequest`].
    pub fn build(self) -> Result<AccountsSwitchChartLedgerRequest, BuildError> {
        Ok(AccountsSwitchChartLedgerRequest {})
    }
}
