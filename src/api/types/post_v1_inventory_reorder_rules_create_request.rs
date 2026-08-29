pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryReorderRulesCreateRequest {
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
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1InventoryReorderRulesCreateRequest {
    pub fn builder() -> PostV1InventoryReorderRulesCreateRequestBuilder {
        <PostV1InventoryReorderRulesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryReorderRulesCreateRequestBuilder {
    item_id: Option<String>,
    warehouse_id: Option<String>,
    min_qty: Option<String>,
    reorder_qty: Option<String>,
    is_active: Option<bool>,
    notes: Option<String>,
}

impl PostV1InventoryReorderRulesCreateRequestBuilder {
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

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryReorderRulesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1InventoryReorderRulesCreateRequestBuilder::item_id)
    /// - [`min_qty`](PostV1InventoryReorderRulesCreateRequestBuilder::min_qty)
    pub fn build(self) -> Result<PostV1InventoryReorderRulesCreateRequest, BuildError> {
        Ok(PostV1InventoryReorderRulesCreateRequest {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            warehouse_id: self.warehouse_id,
            min_qty: self
                .min_qty
                .ok_or_else(|| BuildError::missing_field("min_qty"))?,
            reorder_qty: self.reorder_qty,
            is_active: self.is_active,
            notes: self.notes,
        })
    }
}
