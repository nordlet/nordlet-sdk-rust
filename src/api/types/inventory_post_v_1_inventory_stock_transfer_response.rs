pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockTransferResponse {
    #[serde(rename = "outMovementId")]
    #[serde(default)]
    pub out_movement_id: String,
    #[serde(rename = "inMovementId")]
    #[serde(default)]
    pub in_movement_id: String,
    #[serde(rename = "totalCost")]
    #[serde(default)]
    pub total_cost: String,
}

impl PostV1InventoryStockTransferResponse {
    pub fn builder() -> PostV1InventoryStockTransferResponseBuilder {
        <PostV1InventoryStockTransferResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockTransferResponseBuilder {
    out_movement_id: Option<String>,
    in_movement_id: Option<String>,
    total_cost: Option<String>,
}

impl PostV1InventoryStockTransferResponseBuilder {
    pub fn out_movement_id(mut self, value: impl Into<String>) -> Self {
        self.out_movement_id = Some(value.into());
        self
    }

    pub fn in_movement_id(mut self, value: impl Into<String>) -> Self {
        self.in_movement_id = Some(value.into());
        self
    }

    pub fn total_cost(mut self, value: impl Into<String>) -> Self {
        self.total_cost = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockTransferResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`out_movement_id`](PostV1InventoryStockTransferResponseBuilder::out_movement_id)
    /// - [`in_movement_id`](PostV1InventoryStockTransferResponseBuilder::in_movement_id)
    /// - [`total_cost`](PostV1InventoryStockTransferResponseBuilder::total_cost)
    pub fn build(self) -> Result<PostV1InventoryStockTransferResponse, BuildError> {
        Ok(PostV1InventoryStockTransferResponse {
            out_movement_id: self
                .out_movement_id
                .ok_or_else(|| BuildError::missing_field("out_movement_id"))?,
            in_movement_id: self
                .in_movement_id
                .ok_or_else(|| BuildError::missing_field("in_movement_id"))?,
            total_cost: self
                .total_cost
                .ok_or_else(|| BuildError::missing_field("total_cost"))?,
        })
    }
}
