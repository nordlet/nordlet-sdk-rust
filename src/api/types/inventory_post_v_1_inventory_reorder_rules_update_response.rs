pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryReorderRulesUpdateResponse {
    #[serde(default)]
    pub id: String,
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
    #[serde(default)]
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1InventoryReorderRulesUpdateResponse {
    pub fn builder() -> PostV1InventoryReorderRulesUpdateResponseBuilder {
        <PostV1InventoryReorderRulesUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryReorderRulesUpdateResponseBuilder {
    id: Option<String>,
    item_id: Option<String>,
    warehouse_id: Option<String>,
    min_qty: Option<String>,
    reorder_qty: Option<String>,
    is_active: Option<bool>,
    notes: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1InventoryReorderRulesUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryReorderRulesUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryReorderRulesUpdateResponseBuilder::id)
    /// - [`item_id`](PostV1InventoryReorderRulesUpdateResponseBuilder::item_id)
    /// - [`min_qty`](PostV1InventoryReorderRulesUpdateResponseBuilder::min_qty)
    /// - [`is_active`](PostV1InventoryReorderRulesUpdateResponseBuilder::is_active)
    /// - [`created_at`](PostV1InventoryReorderRulesUpdateResponseBuilder::created_at)
    /// - [`updated_at`](PostV1InventoryReorderRulesUpdateResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1InventoryReorderRulesUpdateResponse, BuildError> {
        Ok(PostV1InventoryReorderRulesUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            warehouse_id: self.warehouse_id,
            min_qty: self
                .min_qty
                .ok_or_else(|| BuildError::missing_field("min_qty"))?,
            reorder_qty: self.reorder_qty,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
