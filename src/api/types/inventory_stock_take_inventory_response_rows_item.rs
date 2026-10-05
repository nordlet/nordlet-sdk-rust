pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockTakeInventoryResponseRowsItem {
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

impl StockTakeInventoryResponseRowsItem {
    pub fn builder() -> StockTakeInventoryResponseRowsItemBuilder {
        <StockTakeInventoryResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockTakeInventoryResponseRowsItemBuilder {
    item_id: Option<String>,
    on_hand: Option<String>,
    counted: Option<String>,
    difference: Option<String>,
    adjustment_cost: Option<String>,
}

impl StockTakeInventoryResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`StockTakeInventoryResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](StockTakeInventoryResponseRowsItemBuilder::item_id)
    /// - [`on_hand`](StockTakeInventoryResponseRowsItemBuilder::on_hand)
    /// - [`counted`](StockTakeInventoryResponseRowsItemBuilder::counted)
    /// - [`difference`](StockTakeInventoryResponseRowsItemBuilder::difference)
    /// - [`adjustment_cost`](StockTakeInventoryResponseRowsItemBuilder::adjustment_cost)
    pub fn build(self) -> Result<StockTakeInventoryResponseRowsItem, BuildError> {
        Ok(StockTakeInventoryResponseRowsItem {
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
