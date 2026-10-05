pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinancialStatementsReportsResponseProfitLoss {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub revenue: String,
    #[serde(default)]
    pub expenses: String,
    #[serde(rename = "netResult")]
    #[serde(default)]
    pub net_result: String,
}

impl FinancialStatementsReportsResponseProfitLoss {
    pub fn builder() -> FinancialStatementsReportsResponseProfitLossBuilder {
        <FinancialStatementsReportsResponseProfitLossBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancialStatementsReportsResponseProfitLossBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    revenue: Option<String>,
    expenses: Option<String>,
    net_result: Option<String>,
}

impl FinancialStatementsReportsResponseProfitLossBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
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

    /// Consumes the builder and constructs a [`FinancialStatementsReportsResponseProfitLoss`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](FinancialStatementsReportsResponseProfitLossBuilder::from_date)
    /// - [`to_date`](FinancialStatementsReportsResponseProfitLossBuilder::to_date)
    /// - [`revenue`](FinancialStatementsReportsResponseProfitLossBuilder::revenue)
    /// - [`expenses`](FinancialStatementsReportsResponseProfitLossBuilder::expenses)
    /// - [`net_result`](FinancialStatementsReportsResponseProfitLossBuilder::net_result)
    pub fn build(self) -> Result<FinancialStatementsReportsResponseProfitLoss, BuildError> {
        Ok(FinancialStatementsReportsResponseProfitLoss {
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
