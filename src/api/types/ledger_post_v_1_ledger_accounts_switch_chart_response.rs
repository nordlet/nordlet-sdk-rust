pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsSwitchChartResponse {
    #[serde(rename = "chartTemplate")]
    #[serde(default)]
    pub chart_template: String,
    #[serde(default)]
    pub accounts: i64,
}

impl PostV1LedgerAccountsSwitchChartResponse {
    pub fn builder() -> PostV1LedgerAccountsSwitchChartResponseBuilder {
        <PostV1LedgerAccountsSwitchChartResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsSwitchChartResponseBuilder {
    chart_template: Option<String>,
    accounts: Option<i64>,
}

impl PostV1LedgerAccountsSwitchChartResponseBuilder {
    pub fn chart_template(mut self, value: impl Into<String>) -> Self {
        self.chart_template = Some(value.into());
        self
    }

    pub fn accounts(mut self, value: i64) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsSwitchChartResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chart_template`](PostV1LedgerAccountsSwitchChartResponseBuilder::chart_template)
    /// - [`accounts`](PostV1LedgerAccountsSwitchChartResponseBuilder::accounts)
    pub fn build(self) -> Result<PostV1LedgerAccountsSwitchChartResponse, BuildError> {
        Ok(PostV1LedgerAccountsSwitchChartResponse {
            chart_template: self
                .chart_template
                .ok_or_else(|| BuildError::missing_field("chart_template"))?,
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
        })
    }
}
