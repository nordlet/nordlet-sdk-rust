pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ReportsFinancialStatementsResponse {
    pub category: PostV1ReportsFinancialStatementsResponseCategory,
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
    pub balance_sheet: PostV1ReportsFinancialStatementsResponseBalanceSheet,
    #[serde(rename = "profitLoss")]
    #[serde(default)]
    pub profit_loss: PostV1ReportsFinancialStatementsResponseProfitLoss,
    #[serde(rename = "balanceSheetDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_sheet_detail: Option<PostV1ReportsFinancialStatementsResponseBalanceSheetDetail>,
    #[serde(rename = "profitLossDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_loss_detail: Option<PostV1ReportsFinancialStatementsResponseProfitLossDetail>,
    #[serde(rename = "equityChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equity_changes: Option<Vec<PostV1ReportsFinancialStatementsResponseEquityChangesItem>>,
    #[serde(rename = "cashFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_flow: Option<PostV1ReportsFinancialStatementsResponseCashFlow>,
}

impl PostV1ReportsFinancialStatementsResponse {
    pub fn builder() -> PostV1ReportsFinancialStatementsResponseBuilder {
        <PostV1ReportsFinancialStatementsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsFinancialStatementsResponseBuilder {
    category: Option<PostV1ReportsFinancialStatementsResponseCategory>,
    layout: Option<String>,
    required_statements: Option<Vec<String>>,
    as_of: Option<String>,
    balance_sheet: Option<PostV1ReportsFinancialStatementsResponseBalanceSheet>,
    profit_loss: Option<PostV1ReportsFinancialStatementsResponseProfitLoss>,
    balance_sheet_detail: Option<PostV1ReportsFinancialStatementsResponseBalanceSheetDetail>,
    profit_loss_detail: Option<PostV1ReportsFinancialStatementsResponseProfitLossDetail>,
    equity_changes: Option<Vec<PostV1ReportsFinancialStatementsResponseEquityChangesItem>>,
    cash_flow: Option<PostV1ReportsFinancialStatementsResponseCashFlow>,
}

impl PostV1ReportsFinancialStatementsResponseBuilder {
    pub fn category(mut self, value: PostV1ReportsFinancialStatementsResponseCategory) -> Self {
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

    pub fn balance_sheet(
        mut self,
        value: PostV1ReportsFinancialStatementsResponseBalanceSheet,
    ) -> Self {
        self.balance_sheet = Some(value);
        self
    }

    pub fn profit_loss(
        mut self,
        value: PostV1ReportsFinancialStatementsResponseProfitLoss,
    ) -> Self {
        self.profit_loss = Some(value);
        self
    }

    pub fn balance_sheet_detail(
        mut self,
        value: PostV1ReportsFinancialStatementsResponseBalanceSheetDetail,
    ) -> Self {
        self.balance_sheet_detail = Some(value);
        self
    }

    pub fn profit_loss_detail(
        mut self,
        value: PostV1ReportsFinancialStatementsResponseProfitLossDetail,
    ) -> Self {
        self.profit_loss_detail = Some(value);
        self
    }

    pub fn equity_changes(
        mut self,
        value: Vec<PostV1ReportsFinancialStatementsResponseEquityChangesItem>,
    ) -> Self {
        self.equity_changes = Some(value);
        self
    }

    pub fn cash_flow(mut self, value: PostV1ReportsFinancialStatementsResponseCashFlow) -> Self {
        self.cash_flow = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsFinancialStatementsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](PostV1ReportsFinancialStatementsResponseBuilder::category)
    /// - [`layout`](PostV1ReportsFinancialStatementsResponseBuilder::layout)
    /// - [`required_statements`](PostV1ReportsFinancialStatementsResponseBuilder::required_statements)
    /// - [`as_of`](PostV1ReportsFinancialStatementsResponseBuilder::as_of)
    /// - [`balance_sheet`](PostV1ReportsFinancialStatementsResponseBuilder::balance_sheet)
    /// - [`profit_loss`](PostV1ReportsFinancialStatementsResponseBuilder::profit_loss)
    pub fn build(self) -> Result<PostV1ReportsFinancialStatementsResponse, BuildError> {
        Ok(PostV1ReportsFinancialStatementsResponse {
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
