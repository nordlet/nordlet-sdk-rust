pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinancialStatementsReportsResponse {
    pub category: FinancialStatementsReportsResponseCategory,
    #[serde(default)]
    pub layout: String,
    #[serde(rename = "requiredStatements")]
    #[serde(default)]
    pub required_statements: Vec<String>,
    #[serde(rename = "asOf")]
    #[serde(default)]
    pub as_of: String,
    #[serde(rename = "balanceSheet")]
    #[serde(default)]
    pub balance_sheet: FinancialStatementsReportsResponseBalanceSheet,
    #[serde(rename = "profitLoss")]
    #[serde(default)]
    pub profit_loss: FinancialStatementsReportsResponseProfitLoss,
    #[serde(rename = "balanceSheetDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_sheet_detail: Option<FinancialStatementsReportsResponseBalanceSheetDetail>,
    #[serde(rename = "profitLossDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_loss_detail: Option<FinancialStatementsReportsResponseProfitLossDetail>,
    #[serde(rename = "equityChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equity_changes: Option<Vec<FinancialStatementsReportsResponseEquityChangesItem>>,
    #[serde(rename = "cashFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_flow: Option<FinancialStatementsReportsResponseCashFlow>,
}

impl FinancialStatementsReportsResponse {
    pub fn builder() -> FinancialStatementsReportsResponseBuilder {
        <FinancialStatementsReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancialStatementsReportsResponseBuilder {
    category: Option<FinancialStatementsReportsResponseCategory>,
    layout: Option<String>,
    required_statements: Option<Vec<String>>,
    as_of: Option<String>,
    balance_sheet: Option<FinancialStatementsReportsResponseBalanceSheet>,
    profit_loss: Option<FinancialStatementsReportsResponseProfitLoss>,
    balance_sheet_detail: Option<FinancialStatementsReportsResponseBalanceSheetDetail>,
    profit_loss_detail: Option<FinancialStatementsReportsResponseProfitLossDetail>,
    equity_changes: Option<Vec<FinancialStatementsReportsResponseEquityChangesItem>>,
    cash_flow: Option<FinancialStatementsReportsResponseCashFlow>,
}

impl FinancialStatementsReportsResponseBuilder {
    pub fn category(mut self, value: FinancialStatementsReportsResponseCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn layout(mut self, value: impl Into<String>) -> Self {
        self.layout = Some(value.into());
        self
    }

    pub fn required_statements(mut self, value: Vec<String>) -> Self {
        self.required_statements = Some(value);
        self
    }

    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.as_of = Some(value.into());
        self
    }

    pub fn balance_sheet(mut self, value: FinancialStatementsReportsResponseBalanceSheet) -> Self {
        self.balance_sheet = Some(value);
        self
    }

    pub fn profit_loss(mut self, value: FinancialStatementsReportsResponseProfitLoss) -> Self {
        self.profit_loss = Some(value);
        self
    }

    pub fn balance_sheet_detail(
        mut self,
        value: FinancialStatementsReportsResponseBalanceSheetDetail,
    ) -> Self {
        self.balance_sheet_detail = Some(value);
        self
    }

    pub fn profit_loss_detail(
        mut self,
        value: FinancialStatementsReportsResponseProfitLossDetail,
    ) -> Self {
        self.profit_loss_detail = Some(value);
        self
    }

    pub fn equity_changes(
        mut self,
        value: Vec<FinancialStatementsReportsResponseEquityChangesItem>,
    ) -> Self {
        self.equity_changes = Some(value);
        self
    }

    pub fn cash_flow(mut self, value: FinancialStatementsReportsResponseCashFlow) -> Self {
        self.cash_flow = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinancialStatementsReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](FinancialStatementsReportsResponseBuilder::category)
    /// - [`layout`](FinancialStatementsReportsResponseBuilder::layout)
    /// - [`required_statements`](FinancialStatementsReportsResponseBuilder::required_statements)
    /// - [`as_of`](FinancialStatementsReportsResponseBuilder::as_of)
    /// - [`balance_sheet`](FinancialStatementsReportsResponseBuilder::balance_sheet)
    /// - [`profit_loss`](FinancialStatementsReportsResponseBuilder::profit_loss)
    pub fn build(self) -> Result<FinancialStatementsReportsResponse, BuildError> {
        Ok(FinancialStatementsReportsResponse {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            layout: self
                .layout
                .ok_or_else(|| BuildError::missing_field("layout"))?,
            required_statements: self
                .required_statements
                .ok_or_else(|| BuildError::missing_field("required_statements"))?,
            as_of: self
                .as_of
                .ok_or_else(|| BuildError::missing_field("as_of"))?,
            balance_sheet: self
                .balance_sheet
                .ok_or_else(|| BuildError::missing_field("balance_sheet"))?,
            profit_loss: self
                .profit_loss
                .ok_or_else(|| BuildError::missing_field("profit_loss"))?,
            balance_sheet_detail: self.balance_sheet_detail,
            profit_loss_detail: self.profit_loss_detail,
            equity_changes: self.equity_changes,
            cash_flow: self.cash_flow,
        })
    }
}
