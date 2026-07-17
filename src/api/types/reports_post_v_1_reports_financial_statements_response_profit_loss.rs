pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsFinancialStatementsResponseProfitLoss {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub revenue: String,
    #[serde(default)]
    pub expenses: String,
    #[serde(rename = "netResult")]
    #[serde(default)]
    pub net_result: String,
}

impl PostV1ReportsFinancialStatementsResponseProfitLoss {
    pub fn builder() -> PostV1ReportsFinancialStatementsResponseProfitLossBuilder {
        <PostV1ReportsFinancialStatementsResponseProfitLossBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsFinancialStatementsResponseProfitLossBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    revenue: Option<String>,
    expenses: Option<String>,
    net_result: Option<String>,
}

impl PostV1ReportsFinancialStatementsResponseProfitLossBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn revenue(mut self, value: impl Into<String>) -> Self {
        self.revenue = Some(value.into());
        self
    }

    pub fn expenses(mut self, value: impl Into<String>) -> Self {
        self.expenses = Some(value.into());
        self
    }

    pub fn net_result(mut self, value: impl Into<String>) -> Self {
        self.net_result = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsFinancialStatementsResponseProfitLoss`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsFinancialStatementsResponseProfitLossBuilder::from_date)
    /// - [`to_date`](PostV1ReportsFinancialStatementsResponseProfitLossBuilder::to_date)
    /// - [`revenue`](PostV1ReportsFinancialStatementsResponseProfitLossBuilder::revenue)
    /// - [`expenses`](PostV1ReportsFinancialStatementsResponseProfitLossBuilder::expenses)
    /// - [`net_result`](PostV1ReportsFinancialStatementsResponseProfitLossBuilder::net_result)
    pub fn build(self) -> Result<PostV1ReportsFinancialStatementsResponseProfitLoss, BuildError> {
        Ok(PostV1ReportsFinancialStatementsResponseProfitLoss {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            revenue: self
                .revenue
                .ok_or_else(|| BuildError::missing_field("revenue"))?,
            expenses: self
                .expenses
                .ok_or_else(|| BuildError::missing_field("expenses"))?,
            net_result: self
                .net_result
                .ok_or_else(|| BuildError::missing_field("net_result"))?,
        })
    }
}
