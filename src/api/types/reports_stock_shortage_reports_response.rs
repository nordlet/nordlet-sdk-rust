pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockShortageReportsResponse {
    #[serde(default)]
    pub rows: Vec<StockShortageReportsResponseRowsItem>,
}

impl StockShortageReportsResponse {
    pub fn builder() -> StockShortageReportsResponseBuilder {
        <StockShortageReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockShortageReportsResponseBuilder {
    rows: Option<Vec<StockShortageReportsResponseRowsItem>>,
}

impl StockShortageReportsResponseBuilder {
    pub fn rows(mut self, value: Vec<StockShortageReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StockShortageReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](StockShortageReportsResponseBuilder::rows)
    pub fn build(self) -> Result<StockShortageReportsResponse, BuildError> {
        Ok(StockShortageReportsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
