pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockTakeRequestLinesItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(rename = "countedQty")]
    #[serde(default)]
    pub counted_qty: String,
    #[serde(rename = "unitCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<String>,
}

impl PostV1InventoryStockTakeRequestLinesItem {
    pub fn builder() -> PostV1InventoryStockTakeRequestLinesItemBuilder {
        <PostV1InventoryStockTakeRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockTakeRequestLinesItemBuilder {
    item_id: Option<String>,
    barcode: Option<String>,
    counted_qty: Option<String>,
    unit_cost: Option<String>,
}

impl PostV1InventoryStockTakeRequestLinesItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn barcode(mut self, value: impl Into<String>) -> Self {
        self.barcode = Some(value.into());
        self
    }

    pub fn counted_qty(mut self, value: impl Into<String>) -> Self {
        self.counted_qty = Some(value.into());
        self
    }

    pub fn unit_cost(mut self, value: impl Into<String>) -> Self {
        self.unit_cost = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockTakeRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`counted_qty`](PostV1InventoryStockTakeRequestLinesItemBuilder::counted_qty)
    pub fn build(self) -> Result<PostV1InventoryStockTakeRequestLinesItem, BuildError> {
        Ok(PostV1InventoryStockTakeRequestLinesItem {
            item_id: self.item_id,
            barcode: self.barcode,
            counted_qty: self
                .counted_qty
                .ok_or_else(|| BuildError::missing_field("counted_qty"))?,
            unit_cost: self.unit_cost,
        })
    }
}
