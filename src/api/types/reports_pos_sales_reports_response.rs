pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PosSalesReportsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<PosSalesReportsResponseRowsItem>,
    #[serde(rename = "byRate")]
    #[serde(default)]
    pub by_rate: Vec<PosSalesReportsResponseByRateItem>,
    #[serde(default)]
    pub totals: PosSalesReportsResponseTotals,
}

impl PosSalesReportsResponse {
    pub fn builder() -> PosSalesReportsResponseBuilder {
        <PosSalesReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PosSalesReportsResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<PosSalesReportsResponseRowsItem>>,
    by_rate: Option<Vec<PosSalesReportsResponseByRateItem>>,
    totals: Option<PosSalesReportsResponseTotals>,
}

impl PosSalesReportsResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<PosSalesReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn by_rate(mut self, value: Vec<PosSalesReportsResponseByRateItem>) -> Self {
        self.by_rate = Some(value);
        self
    }

    pub fn totals(mut self, value: PosSalesReportsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PosSalesReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PosSalesReportsResponseBuilder::from_date)
    /// - [`to_date`](PosSalesReportsResponseBuilder::to_date)
    /// - [`rows`](PosSalesReportsResponseBuilder::rows)
    /// - [`by_rate`](PosSalesReportsResponseBuilder::by_rate)
    /// - [`totals`](PosSalesReportsResponseBuilder::totals)
    pub fn build(self) -> Result<PosSalesReportsResponse, BuildError> {
        Ok(PosSalesReportsResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            by_rate: self
                .by_rate
                .ok_or_else(|| BuildError::missing_field("by_rate"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
