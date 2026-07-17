pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsFinancialStatementsResponseBalanceSheet {
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

impl PostV1ReportsFinancialStatementsResponseBalanceSheet {
    pub fn builder() -> PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder {
        <PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder {
    non_current_assets: Option<String>,
    current_assets: Option<String>,
    total_assets: Option<String>,
    equity: Option<String>,
    of_which_result: Option<String>,
    liabilities: Option<String>,
    total_equity_and_liabilities: Option<String>,
    balanced: Option<bool>,
}

impl PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsFinancialStatementsResponseBalanceSheet`].
    /// This method will fail if any of the following fields are not set:
    /// - [`non_current_assets`](PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder::non_current_assets)
    /// - [`current_assets`](PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder::current_assets)
    /// - [`total_assets`](PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder::total_assets)
    /// - [`equity`](PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder::equity)
    /// - [`of_which_result`](PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder::of_which_result)
    /// - [`liabilities`](PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder::liabilities)
    /// - [`total_equity_and_liabilities`](PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder::total_equity_and_liabilities)
    /// - [`balanced`](PostV1ReportsFinancialStatementsResponseBalanceSheetBuilder::balanced)
    pub fn build(self) -> Result<PostV1ReportsFinancialStatementsResponseBalanceSheet, BuildError> {
        Ok(PostV1ReportsFinancialStatementsResponseBalanceSheet {
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
