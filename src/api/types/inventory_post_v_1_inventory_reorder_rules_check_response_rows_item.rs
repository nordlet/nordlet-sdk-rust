pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryReorderRulesCheckResponseRowsItem {
    #[serde(rename = "ruleId")]
    #[serde(default)]
    pub rule_id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "minQty")]
    #[serde(default)]
    pub min_qty: String,
    #[serde(rename = "reorderQty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reorder_qty: Option<String>,
    #[serde(rename = "onHand")]
    #[serde(default)]
    pub on_hand: String,
    #[serde(default)]
    pub reserved: String,
    #[serde(default)]
    pub available: String,
}

impl PostV1InventoryReorderRulesCheckResponseRowsItem {
    pub fn builder() -> PostV1InventoryReorderRulesCheckResponseRowsItemBuilder {
        <PostV1InventoryReorderRulesCheckResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryReorderRulesCheckResponseRowsItemBuilder {
    rule_id: Option<String>,
    item_id: Option<String>,
    warehouse_id: Option<String>,
    min_qty: Option<String>,
    reorder_qty: Option<String>,
    on_hand: Option<String>,
    reserved: Option<String>,
    available: Option<String>,
}

impl PostV1InventoryReorderRulesCheckResponseRowsItemBuilder {
    pub fn rule_id(mut self, value: impl Into<String>) -> Self {
        self.rule_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn min_qty(mut self, value: impl Into<String>) -> Self {
        self.min_qty = Some(value.into());
        self
    }

    pub fn reorder_qty(mut self, value: impl Into<String>) -> Self {
        self.reorder_qty = Some(value.into());
        self
    }

    pub fn on_hand(mut self, value: impl Into<String>) -> Self {
        self.on_hand = Some(value.into());
        self
    }

    pub fn reserved(mut self, value: impl Into<String>) -> Self {
        self.reserved = Some(value.into());
        self
    }

    pub fn available(mut self, value: impl Into<String>) -> Self {
        self.available = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryReorderRulesCheckResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule_id`](PostV1InventoryReorderRulesCheckResponseRowsItemBuilder::rule_id)
    /// - [`item_id`](PostV1InventoryReorderRulesCheckResponseRowsItemBuilder::item_id)
    /// - [`min_qty`](PostV1InventoryReorderRulesCheckResponseRowsItemBuilder::min_qty)
    /// - [`on_hand`](PostV1InventoryReorderRulesCheckResponseRowsItemBuilder::on_hand)
    /// - [`reserved`](PostV1InventoryReorderRulesCheckResponseRowsItemBuilder::reserved)
    /// - [`available`](PostV1InventoryReorderRulesCheckResponseRowsItemBuilder::available)
    pub fn build(self) -> Result<PostV1InventoryReorderRulesCheckResponseRowsItem, BuildError> {
        Ok(PostV1InventoryReorderRulesCheckResponseRowsItem {
            rule_id: self
                .rule_id
                .ok_or_else(|| BuildError::missing_field("rule_id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            warehouse_id: self.warehouse_id,
            min_qty: self
                .min_qty
                .ok_or_else(|| BuildError::missing_field("min_qty"))?,
            reorder_qty: self.reorder_qty,
            on_hand: self
                .on_hand
                .ok_or_else(|| BuildError::missing_field("on_hand"))?,
            reserved: self
                .reserved
                .ok_or_else(|| BuildError::missing_field("reserved"))?,
            available: self
                .available
                .ok_or_else(|| BuildError::missing_field("available"))?,
        })
    }
}
