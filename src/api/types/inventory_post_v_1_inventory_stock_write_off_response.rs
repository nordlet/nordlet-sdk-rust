pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockWriteOffResponse {
    #[serde(rename = "movementId")]
    #[serde(default)]
    pub movement_id: String,
    #[serde(rename = "totalCost")]
    #[serde(default)]
    pub total_cost: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(default)]
    pub journal_transaction_id: String,
}

impl PostV1InventoryStockWriteOffResponse {
    pub fn builder() -> PostV1InventoryStockWriteOffResponseBuilder {
        <PostV1InventoryStockWriteOffResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockWriteOffResponseBuilder {
    movement_id: Option<String>,
    total_cost: Option<String>,
    journal_transaction_id: Option<String>,
}

impl PostV1InventoryStockWriteOffResponseBuilder {
    pub fn movement_id(mut self, value: impl Into<String>) -> Self {
        self.movement_id = Some(value.into());
        self
    }

    pub fn total_cost(mut self, value: impl Into<String>) -> Self {
        self.total_cost = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockWriteOffResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`movement_id`](PostV1InventoryStockWriteOffResponseBuilder::movement_id)
    /// - [`total_cost`](PostV1InventoryStockWriteOffResponseBuilder::total_cost)
    /// - [`journal_transaction_id`](PostV1InventoryStockWriteOffResponseBuilder::journal_transaction_id)
    pub fn build(self) -> Result<PostV1InventoryStockWriteOffResponse, BuildError> {
        Ok(PostV1InventoryStockWriteOffResponse {
            movement_id: self
                .movement_id
                .ok_or_else(|| BuildError::missing_field("movement_id"))?,
            total_cost: self
                .total_cost
                .ok_or_else(|| BuildError::missing_field("total_cost"))?,
            journal_transaction_id: self
                .journal_transaction_id
                .ok_or_else(|| BuildError::missing_field("journal_transaction_id"))?,
        })
    }
}
