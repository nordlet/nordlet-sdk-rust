pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseStatements {
    pub category: ReportConsolidationResponseStatementsCategory,
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
    pub balance_sheet: ReportConsolidationResponseStatementsBalanceSheet,
    #[serde(rename = "profitLoss")]
    #[serde(default)]
    pub profit_loss: ReportConsolidationResponseStatementsProfitLoss,
    #[serde(rename = "balanceSheetDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_sheet_detail: Option<ReportConsolidationResponseStatementsBalanceSheetDetail>,
    #[serde(rename = "profitLossDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_loss_detail: Option<ReportConsolidationResponseStatementsProfitLossDetail>,
}

impl ReportConsolidationResponseStatements {
    pub fn builder() -> ReportConsolidationResponseStatementsBuilder {
        <ReportConsolidationResponseStatementsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseStatementsBuilder {
    category: Option<ReportConsolidationResponseStatementsCategory>,
    layout: Option<String>,
    required_statements: Option<Vec<String>>,
    as_of: Option<String>,
    balance_sheet: Option<ReportConsolidationResponseStatementsBalanceSheet>,
    profit_loss: Option<ReportConsolidationResponseStatementsProfitLoss>,
    balance_sheet_detail: Option<ReportConsolidationResponseStatementsBalanceSheetDetail>,
    profit_loss_detail: Option<ReportConsolidationResponseStatementsProfitLossDetail>,
}

impl ReportConsolidationResponseStatementsBuilder {
    pub fn category(mut self, value: ReportConsolidationResponseStatementsCategory) -> Self {
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
        value: ReportConsolidationResponseStatementsBalanceSheet,
    ) -> Self {
        self.balance_sheet = Some(value);
        self
    }

    pub fn profit_loss(mut self, value: ReportConsolidationResponseStatementsProfitLoss) -> Self {
        self.profit_loss = Some(value);
        self
    }

    pub fn balance_sheet_detail(
        mut self,
        value: ReportConsolidationResponseStatementsBalanceSheetDetail,
    ) -> Self {
        self.balance_sheet_detail = Some(value);
        self
    }

    pub fn profit_loss_detail(
        mut self,
        value: ReportConsolidationResponseStatementsProfitLossDetail,
    ) -> Self {
        self.profit_loss_detail = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseStatements`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](ReportConsolidationResponseStatementsBuilder::category)
    /// - [`layout`](ReportConsolidationResponseStatementsBuilder::layout)
    /// - [`required_statements`](ReportConsolidationResponseStatementsBuilder::required_statements)
    /// - [`as_of`](ReportConsolidationResponseStatementsBuilder::as_of)
    /// - [`balance_sheet`](ReportConsolidationResponseStatementsBuilder::balance_sheet)
    /// - [`profit_loss`](ReportConsolidationResponseStatementsBuilder::profit_loss)
    pub fn build(self) -> Result<ReportConsolidationResponseStatements, BuildError> {
        Ok(ReportConsolidationResponseStatements {
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
        })
    }
}
