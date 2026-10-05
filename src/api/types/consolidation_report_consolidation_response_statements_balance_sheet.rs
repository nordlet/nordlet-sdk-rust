pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseStatementsBalanceSheet {
    #[serde(rename = "nonCurrentAssets")]
    #[serde(default)]
    pub non_current_assets: String,
    #[serde(rename = "currentAssets")]
    #[serde(default)]
    pub current_assets: String,
    #[serde(rename = "totalAssets")]
    #[serde(default)]
    pub total_assets: String,
    #[serde(default)]
    pub equity: String,
    #[serde(rename = "ofWhichResult")]
    #[serde(default)]
    pub of_which_result: String,
    #[serde(default)]
    pub liabilities: String,
    #[serde(rename = "totalEquityAndLiabilities")]
    #[serde(default)]
    pub total_equity_and_liabilities: String,
    #[serde(default)]
    pub balanced: bool,
}

impl ReportConsolidationResponseStatementsBalanceSheet {
    pub fn builder() -> ReportConsolidationResponseStatementsBalanceSheetBuilder {
        <ReportConsolidationResponseStatementsBalanceSheetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseStatementsBalanceSheetBuilder {
    non_current_assets: Option<String>,
    current_assets: Option<String>,
    total_assets: Option<String>,
    equity: Option<String>,
    of_which_result: Option<String>,
    liabilities: Option<String>,
    total_equity_and_liabilities: Option<String>,
    balanced: Option<bool>,
}

impl ReportConsolidationResponseStatementsBalanceSheetBuilder {
    pub fn non_current_assets(mut self, value: impl Into<String>) -> Self {
        self.non_current_assets = Some(value.into());
        self
    }

    pub fn current_assets(mut self, value: impl Into<String>) -> Self {
        self.current_assets = Some(value.into());
        self
    }

    pub fn total_assets(mut self, value: impl Into<String>) -> Self {
        self.total_assets = Some(value.into());
        self
    }

    pub fn equity(mut self, value: impl Into<String>) -> Self {
        self.equity = Some(value.into());
        self
    }

    pub fn of_which_result(mut self, value: impl Into<String>) -> Self {
        self.of_which_result = Some(value.into());
        self
    }

    pub fn liabilities(mut self, value: impl Into<String>) -> Self {
        self.liabilities = Some(value.into());
        self
    }

    pub fn total_equity_and_liabilities(mut self, value: impl Into<String>) -> Self {
        self.total_equity_and_liabilities = Some(value.into());
        self
    }

    pub fn balanced(mut self, value: bool) -> Self {
        self.balanced = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseStatementsBalanceSheet`].
    /// This method will fail if any of the following fields are not set:
    /// - [`non_current_assets`](ReportConsolidationResponseStatementsBalanceSheetBuilder::non_current_assets)
    /// - [`current_assets`](ReportConsolidationResponseStatementsBalanceSheetBuilder::current_assets)
    /// - [`total_assets`](ReportConsolidationResponseStatementsBalanceSheetBuilder::total_assets)
    /// - [`equity`](ReportConsolidationResponseStatementsBalanceSheetBuilder::equity)
    /// - [`of_which_result`](ReportConsolidationResponseStatementsBalanceSheetBuilder::of_which_result)
    /// - [`liabilities`](ReportConsolidationResponseStatementsBalanceSheetBuilder::liabilities)
    /// - [`total_equity_and_liabilities`](ReportConsolidationResponseStatementsBalanceSheetBuilder::total_equity_and_liabilities)
    /// - [`balanced`](ReportConsolidationResponseStatementsBalanceSheetBuilder::balanced)
    pub fn build(self) -> Result<ReportConsolidationResponseStatementsBalanceSheet, BuildError> {
        Ok(ReportConsolidationResponseStatementsBalanceSheet {
            non_current_assets: self
                .non_current_assets
                .ok_or_else(|| BuildError::missing_field("non_current_assets"))?,
            current_assets: self
                .current_assets
                .ok_or_else(|| BuildError::missing_field("current_assets"))?,
            total_assets: self
                .total_assets
                .ok_or_else(|| BuildError::missing_field("total_assets"))?,
            equity: self
                .equity
                .ok_or_else(|| BuildError::missing_field("equity"))?,
            of_which_result: self
                .of_which_result
                .ok_or_else(|| BuildError::missing_field("of_which_result"))?,
            liabilities: self
                .liabilities
                .ok_or_else(|| BuildError::missing_field("liabilities"))?,
            total_equity_and_liabilities: self
                .total_equity_and_liabilities
                .ok_or_else(|| BuildError::missing_field("total_equity_and_liabilities"))?,
            balanced: self
                .balanced
                .ok_or_else(|| BuildError::missing_field("balanced"))?,
        })
    }
}
