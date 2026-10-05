pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockLevelsInventoryResponse {
    #[serde(default)]
    pub rows: Vec<StockLevelsInventoryResponseRowsItem>,
}

impl StockLevelsInventoryResponse {
    pub fn builder() -> StockLevelsInventoryResponseBuilder {
        <StockLevelsInventoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockLevelsInventoryResponseBuilder {
    rows: Option<Vec<StockLevelsInventoryResponseRowsItem>>,
}

impl StockLevelsInventoryResponseBuilder {
    pub fn rows(mut self, value: Vec<StockLevelsInventoryResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StockLevelsInventoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](StockLevelsInventoryResponseBuilder::rows)
    pub fn build(self) -> Result<StockLevelsInventoryResponse, BuildError> {
        Ok(StockLevelsInventoryResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
