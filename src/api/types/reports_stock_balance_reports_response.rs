pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockBalanceReportsResponse {
    #[serde(rename = "asOf")]
    #[serde(default)]
    pub as_of: String,
    #[serde(default)]
    pub rows: Vec<StockBalanceReportsResponseRowsItem>,
    #[serde(rename = "totalValue")]
    #[serde(default)]
    pub total_value: String,
}

impl StockBalanceReportsResponse {
    pub fn builder() -> StockBalanceReportsResponseBuilder {
        <StockBalanceReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockBalanceReportsResponseBuilder {
    as_of: Option<String>,
    rows: Option<Vec<StockBalanceReportsResponseRowsItem>>,
    total_value: Option<String>,
}

impl StockBalanceReportsResponseBuilder {
    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.as_of = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<StockBalanceReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total_value(mut self, value: impl Into<String>) -> Self {
        self.total_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockBalanceReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of`](StockBalanceReportsResponseBuilder::as_of)
    /// - [`rows`](StockBalanceReportsResponseBuilder::rows)
    /// - [`total_value`](StockBalanceReportsResponseBuilder::total_value)
    pub fn build(self) -> Result<StockBalanceReportsResponse, BuildError> {
        Ok(StockBalanceReportsResponse {
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
