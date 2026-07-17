pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsTrialBalanceResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsTrialBalanceResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1ReportsTrialBalanceResponseTotals,
}

impl PostV1ReportsTrialBalanceResponse {
    pub fn builder() -> PostV1ReportsTrialBalanceResponseBuilder {
        <PostV1ReportsTrialBalanceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsTrialBalanceResponseBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsTrialBalanceResponseRowsItem>>,
    totals: Option<PostV1ReportsTrialBalanceResponseTotals>,
}

impl PostV1ReportsTrialBalanceResponseBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsTrialBalanceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1ReportsTrialBalanceResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsTrialBalanceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsTrialBalanceResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsTrialBalanceResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsTrialBalanceResponseBuilder::rows)
    /// - [`totals`](PostV1ReportsTrialBalanceResponseBuilder::totals)
    pub fn build(self) -> Result<PostV1ReportsTrialBalanceResponse, BuildError> {
        Ok(PostV1ReportsTrialBalanceResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
