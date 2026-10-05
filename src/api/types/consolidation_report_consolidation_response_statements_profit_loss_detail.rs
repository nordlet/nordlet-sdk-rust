pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseStatementsProfitLossDetail {
    #[serde(rename = "salesRevenue")]
    #[serde(default)]
    pub sales_revenue: String,
    #[serde(rename = "costOfSales")]
    #[serde(default)]
    pub cost_of_sales: String,
    #[serde(rename = "grossProfit")]
    #[serde(default)]
    pub gross_profit: String,
    #[serde(rename = "sellingExpenses")]
    #[serde(default)]
    pub selling_expenses: String,
    #[serde(rename = "adminExpenses")]
    #[serde(default)]
    pub admin_expenses: String,
    #[serde(rename = "operatingProfit")]
    #[serde(default)]
    pub operating_profit: String,
    #[serde(rename = "otherActivityResult")]
    #[serde(default)]
    pub other_activity_result: String,
    #[serde(rename = "financialActivityResult")]
    #[serde(default)]
    pub financial_activity_result: String,
    #[serde(rename = "profitBeforeTax")]
    #[serde(default)]
    pub profit_before_tax: String,
    #[serde(rename = "incomeTax")]
    #[serde(default)]
    pub income_tax: String,
    #[serde(rename = "netProfit")]
    #[serde(default)]
    pub net_profit: String,
}

impl ReportConsolidationResponseStatementsProfitLossDetail {
    pub fn builder() -> ReportConsolidationResponseStatementsProfitLossDetailBuilder {
        <ReportConsolidationResponseStatementsProfitLossDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseStatementsProfitLossDetailBuilder {
    sales_revenue: Option<String>,
    cost_of_sales: Option<String>,
    gross_profit: Option<String>,
    selling_expenses: Option<String>,
    admin_expenses: Option<String>,
    operating_profit: Option<String>,
    other_activity_result: Option<String>,
    financial_activity_result: Option<String>,
    profit_before_tax: Option<String>,
    income_tax: Option<String>,
    net_profit: Option<String>,
}

impl ReportConsolidationResponseStatementsProfitLossDetailBuilder {
    pub fn sales_revenue(mut self, value: impl Into<String>) -> Self {
        self.sales_revenue = Some(value.into());
        self
    }

    pub fn cost_of_sales(mut self, value: impl Into<String>) -> Self {
        self.cost_of_sales = Some(value.into());
        self
    }

    pub fn gross_profit(mut self, value: impl Into<String>) -> Self {
        self.gross_profit = Some(value.into());
        self
    }

    pub fn selling_expenses(mut self, value: impl Into<String>) -> Self {
        self.selling_expenses = Some(value.into());
        self
    }

    pub fn admin_expenses(mut self, value: impl Into<String>) -> Self {
        self.admin_expenses = Some(value.into());
        self
    }

    pub fn operating_profit(mut self, value: impl Into<String>) -> Self {
        self.operating_profit = Some(value.into());
        self
    }

    pub fn other_activity_result(mut self, value: impl Into<String>) -> Self {
        self.other_activity_result = Some(value.into());
        self
    }

    pub fn financial_activity_result(mut self, value: impl Into<String>) -> Self {
        self.financial_activity_result = Some(value.into());
        self
    }

    pub fn profit_before_tax(mut self, value: impl Into<String>) -> Self {
        self.profit_before_tax = Some(value.into());
        self
    }

    pub fn income_tax(mut self, value: impl Into<String>) -> Self {
        self.income_tax = Some(value.into());
        self
    }

    pub fn net_profit(mut self, value: impl Into<String>) -> Self {
        self.net_profit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseStatementsProfitLossDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sales_revenue`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::sales_revenue)
    /// - [`cost_of_sales`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::cost_of_sales)
    /// - [`gross_profit`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::gross_profit)
    /// - [`selling_expenses`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::selling_expenses)
    /// - [`admin_expenses`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::admin_expenses)
    /// - [`operating_profit`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::operating_profit)
    /// - [`other_activity_result`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::other_activity_result)
    /// - [`financial_activity_result`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::financial_activity_result)
    /// - [`profit_before_tax`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::profit_before_tax)
    /// - [`income_tax`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::income_tax)
    /// - [`net_profit`](ReportConsolidationResponseStatementsProfitLossDetailBuilder::net_profit)
    pub fn build(
        self,
    ) -> Result<ReportConsolidationResponseStatementsProfitLossDetail, BuildError> {
        Ok(ReportConsolidationResponseStatementsProfitLossDetail {
            sales_revenue: self
                .sales_revenue
                .ok_or_else(|| BuildError::missing_field("sales_revenue"))?,
            cost_of_sales: self
                .cost_of_sales
                .ok_or_else(|| BuildError::missing_field("cost_of_sales"))?,
            gross_profit: self
                .gross_profit
                .ok_or_else(|| BuildError::missing_field("gross_profit"))?,
            selling_expenses: self
                .selling_expenses
                .ok_or_else(|| BuildError::missing_field("selling_expenses"))?,
            admin_expenses: self
                .admin_expenses
                .ok_or_else(|| BuildError::missing_field("admin_expenses"))?,
            operating_profit: self
                .operating_profit
                .ok_or_else(|| BuildError::missing_field("operating_profit"))?,
            other_activity_result: self
                .other_activity_result
                .ok_or_else(|| BuildError::missing_field("other_activity_result"))?,
            financial_activity_result: self
                .financial_activity_result
                .ok_or_else(|| BuildError::missing_field("financial_activity_result"))?,
            profit_before_tax: self
                .profit_before_tax
                .ok_or_else(|| BuildError::missing_field("profit_before_tax"))?,
            income_tax: self
                .income_tax
                .ok_or_else(|| BuildError::missing_field("income_tax"))?,
            net_profit: self
                .net_profit
                .ok_or_else(|| BuildError::missing_field("net_profit"))?,
        })
    }
}
