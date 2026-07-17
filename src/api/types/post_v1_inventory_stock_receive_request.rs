pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockReceiveRequest {
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitCost")]
    #[serde(default)]
    pub unit_cost: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1InventoryStockReceiveRequest {
    pub fn builder() -> PostV1InventoryStockReceiveRequestBuilder {
        <PostV1InventoryStockReceiveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockReceiveRequestBuilder {
    warehouse_id: Option<String>,
    item_id: Option<String>,
    date: Option<String>,
    quantity: Option<String>,
    unit_cost: Option<String>,
    notes: Option<String>,
}

impl PostV1InventoryStockReceiveRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit_cost(mut self, value: impl Into<String>) -> Self {
        self.unit_cost = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockReceiveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`warehouse_id`](PostV1InventoryStockReceiveRequestBuilder::warehouse_id)
    /// - [`item_id`](PostV1InventoryStockReceiveRequestBuilder::item_id)
    /// - [`date`](PostV1InventoryStockReceiveRequestBuilder::date)
    /// - [`quantity`](PostV1InventoryStockReceiveRequestBuilder::quantity)
    /// - [`unit_cost`](PostV1InventoryStockReceiveRequestBuilder::unit_cost)
    pub fn build(self) -> Result<PostV1InventoryStockReceiveRequest, BuildError> {
        Ok(PostV1InventoryStockReceiveRequest {
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_cost: self
                .unit_cost
                .ok_or_else(|| BuildError::missing_field("unit_cost"))?,
            notes: self.notes,
        })
    }
}
