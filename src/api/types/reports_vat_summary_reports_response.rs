pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatSummaryReportsResponse {
    #[serde(default)]
    pub side: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<VatSummaryReportsResponseRowsItem>,
    #[serde(default)]
    pub totals: VatSummaryReportsResponseTotals,
}

impl VatSummaryReportsResponse {
    pub fn builder() -> VatSummaryReportsResponseBuilder {
        <VatSummaryReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSummaryReportsResponseBuilder {
    side: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<VatSummaryReportsResponseRowsItem>>,
    totals: Option<VatSummaryReportsResponseTotals>,
}

impl VatSummaryReportsResponseBuilder {
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<VatSummaryReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: VatSummaryReportsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatSummaryReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`side`](VatSummaryReportsResponseBuilder::side)
    /// - [`from_date`](VatSummaryReportsResponseBuilder::from_date)
    /// - [`to_date`](VatSummaryReportsResponseBuilder::to_date)
    /// - [`rows`](VatSummaryReportsResponseBuilder::rows)
    /// - [`totals`](VatSummaryReportsResponseBuilder::totals)
    pub fn build(self) -> Result<VatSummaryReportsResponse, BuildError> {
        Ok(VatSummaryReportsResponse {
            side: self.side.ok_or_else(|| BuildError::missing_field("side"))?,
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
