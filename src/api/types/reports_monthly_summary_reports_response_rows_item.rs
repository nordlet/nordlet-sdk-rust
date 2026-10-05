pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MonthlySummaryReportsResponseRowsItem {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub receivables: String,
    #[serde(default)]
    pub payables: String,
    #[serde(default)]
    pub revenue: String,
    #[serde(default)]
    pub expenses: String,
    #[serde(rename = "netResult")]
    #[serde(default)]
    pub net_result: String,
}

impl MonthlySummaryReportsResponseRowsItem {
    pub fn builder() -> MonthlySummaryReportsResponseRowsItemBuilder {
        <MonthlySummaryReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MonthlySummaryReportsResponseRowsItemBuilder {
    year: Option<i64>,
    month: Option<i64>,
    receivables: Option<String>,
    payables: Option<String>,
    revenue: Option<String>,
    expenses: Option<String>,
    net_result: Option<String>,
}

impl MonthlySummaryReportsResponseRowsItemBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn receivables(mut self, value: impl Into<String>) -> Self {
        self.receivables = Some(value.into());
        self
    }

    pub fn payables(mut self, value: impl Into<String>) -> Self {
        self.payables = Some(value.into());
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

    /// Consumes the builder and constructs a [`MonthlySummaryReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](MonthlySummaryReportsResponseRowsItemBuilder::year)
    /// - [`month`](MonthlySummaryReportsResponseRowsItemBuilder::month)
    /// - [`receivables`](MonthlySummaryReportsResponseRowsItemBuilder::receivables)
    /// - [`payables`](MonthlySummaryReportsResponseRowsItemBuilder::payables)
    /// - [`revenue`](MonthlySummaryReportsResponseRowsItemBuilder::revenue)
    /// - [`expenses`](MonthlySummaryReportsResponseRowsItemBuilder::expenses)
    /// - [`net_result`](MonthlySummaryReportsResponseRowsItemBuilder::net_result)
    pub fn build(self) -> Result<MonthlySummaryReportsResponseRowsItem, BuildError> {
        Ok(MonthlySummaryReportsResponseRowsItem {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            receivables: self
                .receivables
                .ok_or_else(|| BuildError::missing_field("receivables"))?,
            payables: self
                .payables
                .ok_or_else(|| BuildError::missing_field("payables"))?,
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
