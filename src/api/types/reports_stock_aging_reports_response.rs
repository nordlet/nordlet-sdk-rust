pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockAgingReportsResponse {
    #[serde(rename = "asOf")]
    #[serde(default)]
    pub as_of: String,
    #[serde(default)]
    pub rows: Vec<StockAgingReportsResponseRowsItem>,
    #[serde(rename = "totalValue")]
    #[serde(default)]
    pub total_value: String,
}

impl StockAgingReportsResponse {
    pub fn builder() -> StockAgingReportsResponseBuilder {
        <StockAgingReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockAgingReportsResponseBuilder {
    as_of: Option<String>,
    rows: Option<Vec<StockAgingReportsResponseRowsItem>>,
    total_value: Option<String>,
}

impl StockAgingReportsResponseBuilder {
    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.as_of = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<StockAgingReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total_value(mut self, value: impl Into<String>) -> Self {
        self.total_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockAgingReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of`](StockAgingReportsResponseBuilder::as_of)
    /// - [`rows`](StockAgingReportsResponseBuilder::rows)
    /// - [`total_value`](StockAgingReportsResponseBuilder::total_value)
    pub fn build(self) -> Result<StockAgingReportsResponse, BuildError> {
        Ok(StockAgingReportsResponse {
            as_of: self
                .as_of
                .ok_or_else(|| BuildError::missing_field("as_of"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total_value: self
                .total_value
                .ok_or_else(|| BuildError::missing_field("total_value"))?,
        })
    }
}
