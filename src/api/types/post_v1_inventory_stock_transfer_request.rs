pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockTransferRequest {
    #[serde(rename = "fromWarehouseId")]
    #[serde(default)]
    pub from_warehouse_id: String,
    #[serde(rename = "toWarehouseId")]
    #[serde(default)]
    pub to_warehouse_id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1InventoryStockTransferRequest {
    pub fn builder() -> PostV1InventoryStockTransferRequestBuilder {
        <PostV1InventoryStockTransferRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockTransferRequestBuilder {
    from_warehouse_id: Option<String>,
    to_warehouse_id: Option<String>,
    item_id: Option<String>,
    date: Option<String>,
    quantity: Option<String>,
    notes: Option<String>,
}

impl PostV1InventoryStockTransferRequestBuilder {
    pub fn from_warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.from_warehouse_id = Some(value.into());
        self
    }

    pub fn to_warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.to_warehouse_id = Some(value.into());
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

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockTransferRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_warehouse_id`](PostV1InventoryStockTransferRequestBuilder::from_warehouse_id)
    /// - [`to_warehouse_id`](PostV1InventoryStockTransferRequestBuilder::to_warehouse_id)
    /// - [`item_id`](PostV1InventoryStockTransferRequestBuilder::item_id)
    /// - [`date`](PostV1InventoryStockTransferRequestBuilder::date)
    /// - [`quantity`](PostV1InventoryStockTransferRequestBuilder::quantity)
    pub fn build(self) -> Result<PostV1InventoryStockTransferRequest, BuildError> {
        Ok(PostV1InventoryStockTransferRequest {
            from_warehouse_id: self
                .from_warehouse_id
                .ok_or_else(|| BuildError::missing_field("from_warehouse_id"))?,
            to_warehouse_id: self
                .to_warehouse_id
                .ok_or_else(|| BuildError::missing_field("to_warehouse_id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            notes: self.notes,
        })
    }
}
