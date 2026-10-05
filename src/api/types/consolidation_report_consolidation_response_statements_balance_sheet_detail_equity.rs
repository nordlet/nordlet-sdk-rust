pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseStatementsBalanceSheetDetailEquity {
    #[serde(default)]
    pub capital: String,
    #[serde(default)]
    pub reserves: String,
    #[serde(rename = "retainedEarnings")]
    #[serde(default)]
    pub retained_earnings: String,
    #[serde(rename = "otherEquity")]
    #[serde(default)]
    pub other_equity: String,
    #[serde(rename = "periodResult")]
    #[serde(default)]
    pub period_result: String,
    #[serde(default)]
    pub total: String,
}

impl ReportConsolidationResponseStatementsBalanceSheetDetailEquity {
    pub fn builder() -> ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder {
        <ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder {
    capital: Option<String>,
    reserves: Option<String>,
    retained_earnings: Option<String>,
    other_equity: Option<String>,
    period_result: Option<String>,
    total: Option<String>,
}

impl ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder {
    pub fn capital(mut self, value: impl Into<String>) -> Self {
        self.capital = Some(value.into());
        self
    }

    pub fn reserves(mut self, value: impl Into<String>) -> Self {
        self.reserves = Some(value.into());
        self
    }

    pub fn retained_earnings(mut self, value: impl Into<String>) -> Self {
        self.retained_earnings = Some(value.into());
        self
    }

    pub fn other_equity(mut self, value: impl Into<String>) -> Self {
        self.other_equity = Some(value.into());
        self
    }

    pub fn period_result(mut self, value: impl Into<String>) -> Self {
        self.period_result = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseStatementsBalanceSheetDetailEquity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capital`](ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder::capital)
    /// - [`reserves`](ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder::reserves)
    /// - [`retained_earnings`](ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder::retained_earnings)
    /// - [`other_equity`](ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder::other_equity)
    /// - [`period_result`](ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder::period_result)
    /// - [`total`](ReportConsolidationResponseStatementsBalanceSheetDetailEquityBuilder::total)
    pub fn build(
        self,
    ) -> Result<ReportConsolidationResponseStatementsBalanceSheetDetailEquity, BuildError> {
        Ok(
            ReportConsolidationResponseStatementsBalanceSheetDetailEquity {
                capital: self
                    .capital
                    .ok_or_else(|| BuildError::missing_field("capital"))?,
                reserves: self
                    .reserves
                    .ok_or_else(|| BuildError::missing_field("reserves"))?,
                retained_earnings: self
                    .retained_earnings
                    .ok_or_else(|| BuildError::missing_field("retained_earnings"))?,
                other_equity: self
                    .other_equity
                    .ok_or_else(|| BuildError::missing_field("other_equity"))?,
                period_result: self
                    .period_result
                    .ok_or_else(|| BuildError::missing_field("period_result"))?,
                total: self
                    .total
                    .ok_or_else(|| BuildError::missing_field("total"))?,
            },
        )
    }
}
