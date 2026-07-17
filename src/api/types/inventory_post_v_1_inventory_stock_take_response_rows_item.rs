pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockTakeResponseRowsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "onHand")]
    #[serde(default)]
    pub on_hand: String,
    #[serde(default)]
    pub counted: String,
    #[serde(default)]
    pub difference: String,
    #[serde(rename = "adjustmentCost")]
    #[serde(default)]
    pub adjustment_cost: String,
}

impl PostV1InventoryStockTakeResponseRowsItem {
    pub fn builder() -> PostV1InventoryStockTakeResponseRowsItemBuilder {
        <PostV1InventoryStockTakeResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockTakeResponseRowsItemBuilder {
    item_id: Option<String>,
    on_hand: Option<String>,
    counted: Option<String>,
    difference: Option<String>,
    adjustment_cost: Option<String>,
}

impl PostV1InventoryStockTakeResponseRowsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn on_hand(mut self, value: impl Into<String>) -> Self {
        self.on_hand = Some(value.into());
        self
    }

    pub fn counted(mut self, value: impl Into<String>) -> Self {
        self.counted = Some(value.into());
        self
    }

    pub fn difference(mut self, value: impl Into<String>) -> Self {
        self.difference = Some(value.into());
        self
    }

    pub fn adjustment_cost(mut self, value: impl Into<String>) -> Self {
        self.adjustment_cost = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockTakeResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1InventoryStockTakeResponseRowsItemBuilder::item_id)
    /// - [`on_hand`](PostV1InventoryStockTakeResponseRowsItemBuilder::on_hand)
    /// - [`counted`](PostV1InventoryStockTakeResponseRowsItemBuilder::counted)
    /// - [`difference`](PostV1InventoryStockTakeResponseRowsItemBuilder::difference)
    /// - [`adjustment_cost`](PostV1InventoryStockTakeResponseRowsItemBuilder::adjustment_cost)
    pub fn build(self) -> Result<PostV1InventoryStockTakeResponseRowsItem, BuildError> {
        Ok(PostV1InventoryStockTakeResponseRowsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            on_hand: self
                .on_hand
                .ok_or_else(|| BuildError::missing_field("on_hand"))?,
            counted: self
                .counted
                .ok_or_else(|| BuildError::missing_field("counted"))?,
            difference: self
                .difference
                .ok_or_else(|| BuildError::missing_field("difference"))?,
            adjustment_cost: self
                .adjustment_cost
                .ok_or_else(|| BuildError::missing_field("adjustment_cost"))?,
        })
    }
}
