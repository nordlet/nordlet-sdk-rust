pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsMonthlySummaryResponseRowsItem {
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

impl PostV1ReportsMonthlySummaryResponseRowsItem {
    pub fn builder() -> PostV1ReportsMonthlySummaryResponseRowsItemBuilder {
        <PostV1ReportsMonthlySummaryResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsMonthlySummaryResponseRowsItemBuilder {
    year: Option<i64>,
    month: Option<i64>,
    receivables: Option<String>,
    payables: Option<String>,
    revenue: Option<String>,
    expenses: Option<String>,
    net_result: Option<String>,
}

impl PostV1ReportsMonthlySummaryResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsMonthlySummaryResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1ReportsMonthlySummaryResponseRowsItemBuilder::year)
    /// - [`month`](PostV1ReportsMonthlySummaryResponseRowsItemBuilder::month)
    /// - [`receivables`](PostV1ReportsMonthlySummaryResponseRowsItemBuilder::receivables)
    /// - [`payables`](PostV1ReportsMonthlySummaryResponseRowsItemBuilder::payables)
    /// - [`revenue`](PostV1ReportsMonthlySummaryResponseRowsItemBuilder::revenue)
    /// - [`expenses`](PostV1ReportsMonthlySummaryResponseRowsItemBuilder::expenses)
    /// - [`net_result`](PostV1ReportsMonthlySummaryResponseRowsItemBuilder::net_result)
    pub fn build(self) -> Result<PostV1ReportsMonthlySummaryResponseRowsItem, BuildError> {
        Ok(PostV1ReportsMonthlySummaryResponseRowsItem {
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
