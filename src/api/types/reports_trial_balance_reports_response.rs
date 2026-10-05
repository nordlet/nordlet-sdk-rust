pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TrialBalanceReportsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<TrialBalanceReportsResponseRowsItem>,
    #[serde(default)]
    pub totals: TrialBalanceReportsResponseTotals,
}

impl TrialBalanceReportsResponse {
    pub fn builder() -> TrialBalanceReportsResponseBuilder {
        <TrialBalanceReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrialBalanceReportsResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<TrialBalanceReportsResponseRowsItem>>,
    totals: Option<TrialBalanceReportsResponseTotals>,
}

impl TrialBalanceReportsResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<TrialBalanceReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: TrialBalanceReportsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TrialBalanceReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](TrialBalanceReportsResponseBuilder::from_date)
    /// - [`to_date`](TrialBalanceReportsResponseBuilder::to_date)
    /// - [`rows`](TrialBalanceReportsResponseBuilder::rows)
    /// - [`totals`](TrialBalanceReportsResponseBuilder::totals)
    pub fn build(self) -> Result<TrialBalanceReportsResponse, BuildError> {
        Ok(TrialBalanceReportsResponse {
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
