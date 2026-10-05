pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuPurchasesReportsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<EuPurchasesReportsResponseRowsItem>,
    #[serde(default)]
    pub totals: EuPurchasesReportsResponseTotals,
}

impl EuPurchasesReportsResponse {
    pub fn builder() -> EuPurchasesReportsResponseBuilder {
        <EuPurchasesReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuPurchasesReportsResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<EuPurchasesReportsResponseRowsItem>>,
    totals: Option<EuPurchasesReportsResponseTotals>,
}

impl EuPurchasesReportsResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<EuPurchasesReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: EuPurchasesReportsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuPurchasesReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](EuPurchasesReportsResponseBuilder::from_date)
    /// - [`to_date`](EuPurchasesReportsResponseBuilder::to_date)
    /// - [`rows`](EuPurchasesReportsResponseBuilder::rows)
    /// - [`totals`](EuPurchasesReportsResponseBuilder::totals)
    pub fn build(self) -> Result<EuPurchasesReportsResponse, BuildError> {
        Ok(EuPurchasesReportsResponse {
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
