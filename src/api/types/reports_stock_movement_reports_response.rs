pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockMovementReportsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<StockMovementReportsResponseRowsItem>,
}

impl StockMovementReportsResponse {
    pub fn builder() -> StockMovementReportsResponseBuilder {
        <StockMovementReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockMovementReportsResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<StockMovementReportsResponseRowsItem>>,
}

impl StockMovementReportsResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<StockMovementReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StockMovementReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](StockMovementReportsResponseBuilder::from_date)
    /// - [`to_date`](StockMovementReportsResponseBuilder::to_date)
    /// - [`rows`](StockMovementReportsResponseBuilder::rows)
    pub fn build(self) -> Result<StockMovementReportsResponse, BuildError> {
        Ok(StockMovementReportsResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
