pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockReceiveInventoryResponse {
    #[serde(rename = "movementId")]
    #[serde(default)]
    pub movement_id: String,
    #[serde(rename = "totalCost")]
    #[serde(default)]
    pub total_cost: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(default)]
    pub warnings: Vec<StockReceiveInventoryResponseWarningsItem>,
}

impl StockReceiveInventoryResponse {
    pub fn builder() -> StockReceiveInventoryResponseBuilder {
        <StockReceiveInventoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockReceiveInventoryResponseBuilder {
    movement_id: Option<String>,
    total_cost: Option<String>,
    journal_transaction_id: Option<String>,
    warnings: Option<Vec<StockReceiveInventoryResponseWarningsItem>>,
}

impl StockReceiveInventoryResponseBuilder {
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

    pub fn warnings(mut self, value: Vec<StockReceiveInventoryResponseWarningsItem>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StockReceiveInventoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`movement_id`](StockReceiveInventoryResponseBuilder::movement_id)
    /// - [`total_cost`](StockReceiveInventoryResponseBuilder::total_cost)
    /// - [`warnings`](StockReceiveInventoryResponseBuilder::warnings)
    pub fn build(self) -> Result<StockReceiveInventoryResponse, BuildError> {
        Ok(StockReceiveInventoryResponse {
            movement_id: self
                .movement_id
                .ok_or_else(|| BuildError::missing_field("movement_id"))?,
            total_cost: self
                .total_cost
                .ok_or_else(|| BuildError::missing_field("total_cost"))?,
            journal_transaction_id: self.journal_transaction_id,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
