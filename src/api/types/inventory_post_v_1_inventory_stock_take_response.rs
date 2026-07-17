pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockTakeResponse {
    #[serde(default)]
    pub rows: Vec<PostV1InventoryStockTakeResponseRowsItem>,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
}

impl PostV1InventoryStockTakeResponse {
    pub fn builder() -> PostV1InventoryStockTakeResponseBuilder {
        <PostV1InventoryStockTakeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockTakeResponseBuilder {
    rows: Option<Vec<PostV1InventoryStockTakeResponseRowsItem>>,
    journal_transaction_id: Option<String>,
}

impl PostV1InventoryStockTakeResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1InventoryStockTakeResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockTakeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1InventoryStockTakeResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1InventoryStockTakeResponse, BuildError> {
        Ok(PostV1InventoryStockTakeResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            journal_transaction_id: self.journal_transaction_id,
        })
    }
}
