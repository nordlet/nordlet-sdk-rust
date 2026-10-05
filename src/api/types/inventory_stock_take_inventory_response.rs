pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockTakeInventoryResponse {
    #[serde(default)]
    pub rows: Vec<StockTakeInventoryResponseRowsItem>,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
}

impl StockTakeInventoryResponse {
    pub fn builder() -> StockTakeInventoryResponseBuilder {
        <StockTakeInventoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockTakeInventoryResponseBuilder {
    rows: Option<Vec<StockTakeInventoryResponseRowsItem>>,
    journal_transaction_id: Option<String>,
}

impl StockTakeInventoryResponseBuilder {
    pub fn rows(mut self, value: Vec<StockTakeInventoryResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockTakeInventoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](StockTakeInventoryResponseBuilder::rows)
    pub fn build(self) -> Result<StockTakeInventoryResponse, BuildError> {
        Ok(StockTakeInventoryResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            journal_transaction_id: self.journal_transaction_id,
        })
    }
}
