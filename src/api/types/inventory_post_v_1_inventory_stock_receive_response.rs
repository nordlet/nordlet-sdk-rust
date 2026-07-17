pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockReceiveResponse {
    #[serde(rename = "movementId")]
    #[serde(default)]
    pub movement_id: String,
    #[serde(rename = "totalCost")]
    #[serde(default)]
    pub total_cost: String,
}

impl PostV1InventoryStockReceiveResponse {
    pub fn builder() -> PostV1InventoryStockReceiveResponseBuilder {
        <PostV1InventoryStockReceiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockReceiveResponseBuilder {
    movement_id: Option<String>,
    total_cost: Option<String>,
}

impl PostV1InventoryStockReceiveResponseBuilder {
    pub fn movement_id(mut self, value: impl Into<String>) -> Self {
        self.movement_id = Some(value.into());
        self
    }

    pub fn total_cost(mut self, value: impl Into<String>) -> Self {
        self.total_cost = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockReceiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`movement_id`](PostV1InventoryStockReceiveResponseBuilder::movement_id)
    /// - [`total_cost`](PostV1InventoryStockReceiveResponseBuilder::total_cost)
    pub fn build(self) -> Result<PostV1InventoryStockReceiveResponse, BuildError> {
        Ok(PostV1InventoryStockReceiveResponse {
            movement_id: self
                .movement_id
                .ok_or_else(|| BuildError::missing_field("movement_id"))?,
            total_cost: self
                .total_cost
                .ok_or_else(|| BuildError::missing_field("total_cost"))?,
        })
    }
}
