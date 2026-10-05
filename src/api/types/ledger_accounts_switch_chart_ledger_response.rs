pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsSwitchChartLedgerResponse {
    #[serde(rename = "chartTemplate")]
    #[serde(default)]
    pub chart_template: String,
    #[serde(default)]
    pub accounts: i64,
}

impl AccountsSwitchChartLedgerResponse {
    pub fn builder() -> AccountsSwitchChartLedgerResponseBuilder {
        <AccountsSwitchChartLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsSwitchChartLedgerResponseBuilder {
    chart_template: Option<String>,
    accounts: Option<i64>,
}

impl AccountsSwitchChartLedgerResponseBuilder {
    pub fn chart_template(mut self, value: impl Into<String>) -> Self {
        self.chart_template = Some(value.into());
        self
    }

    pub fn accounts(mut self, value: i64) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsSwitchChartLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chart_template`](AccountsSwitchChartLedgerResponseBuilder::chart_template)
    /// - [`accounts`](AccountsSwitchChartLedgerResponseBuilder::accounts)
    pub fn build(self) -> Result<AccountsSwitchChartLedgerResponse, BuildError> {
        Ok(AccountsSwitchChartLedgerResponse {
            chart_template: self
                .chart_template
                .ok_or_else(|| BuildError::missing_field("chart_template"))?,
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
        })
    }
}
