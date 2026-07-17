pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseStatements {
    pub category: PostV1ConsolidationReportResponseStatementsCategory,
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
    pub balance_sheet: PostV1ConsolidationReportResponseStatementsBalanceSheet,
    #[serde(rename = "profitLoss")]
    #[serde(default)]
    pub profit_loss: PostV1ConsolidationReportResponseStatementsProfitLoss,
    #[serde(rename = "balanceSheetDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_sheet_detail: Option<PostV1ConsolidationReportResponseStatementsBalanceSheetDetail>,
    #[serde(rename = "profitLossDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_loss_detail: Option<PostV1ConsolidationReportResponseStatementsProfitLossDetail>,
}

impl PostV1ConsolidationReportResponseStatements {
    pub fn builder() -> PostV1ConsolidationReportResponseStatementsBuilder {
        <PostV1ConsolidationReportResponseStatementsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseStatementsBuilder {
    category: Option<PostV1ConsolidationReportResponseStatementsCategory>,
    layout: Option<String>,
    required_statements: Option<Vec<String>>,
    as_of: Option<String>,
    balance_sheet: Option<PostV1ConsolidationReportResponseStatementsBalanceSheet>,
    profit_loss: Option<PostV1ConsolidationReportResponseStatementsProfitLoss>,
    balance_sheet_detail: Option<PostV1ConsolidationReportResponseStatementsBalanceSheetDetail>,
    profit_loss_detail: Option<PostV1ConsolidationReportResponseStatementsProfitLossDetail>,
}

impl PostV1ConsolidationReportResponseStatementsBuilder {
    pub fn category(mut self, value: PostV1ConsolidationReportResponseStatementsCategory) -> Self {
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
        value: PostV1ConsolidationReportResponseStatementsBalanceSheet,
    ) -> Self {
        self.balance_sheet = Some(value);
        self
    }

    pub fn profit_loss(
        mut self,
        value: PostV1ConsolidationReportResponseStatementsProfitLoss,
    ) -> Self {
        self.profit_loss = Some(value);
        self
    }

    pub fn balance_sheet_detail(
        mut self,
        value: PostV1ConsolidationReportResponseStatementsBalanceSheetDetail,
    ) -> Self {
        self.balance_sheet_detail = Some(value);
        self
    }

    pub fn profit_loss_detail(
        mut self,
        value: PostV1ConsolidationReportResponseStatementsProfitLossDetail,
    ) -> Self {
        self.profit_loss_detail = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseStatements`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](PostV1ConsolidationReportResponseStatementsBuilder::category)
    /// - [`layout`](PostV1ConsolidationReportResponseStatementsBuilder::layout)
    /// - [`required_statements`](PostV1ConsolidationReportResponseStatementsBuilder::required_statements)
    /// - [`as_of`](PostV1ConsolidationReportResponseStatementsBuilder::as_of)
    /// - [`balance_sheet`](PostV1ConsolidationReportResponseStatementsBuilder::balance_sheet)
    /// - [`profit_loss`](PostV1ConsolidationReportResponseStatementsBuilder::profit_loss)
    pub fn build(self) -> Result<PostV1ConsolidationReportResponseStatements, BuildError> {
        Ok(PostV1ConsolidationReportResponseStatements {
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
