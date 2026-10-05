pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReorderRulesListInventoryResponseRowsItem {
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ReorderRulesListInventoryResponseRowsItem {
    pub fn builder() -> ReorderRulesListInventoryResponseRowsItemBuilder {
        <ReorderRulesListInventoryResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReorderRulesListInventoryResponseRowsItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    warehouse_id: Option<String>,
    min_qty: Option<String>,
    reorder_qty: Option<String>,
    is_active: Option<bool>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ReorderRulesListInventoryResponseRowsItemBuilder {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReorderRulesListInventoryResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReorderRulesListInventoryResponseRowsItemBuilder::id)
    /// - [`item_id`](ReorderRulesListInventoryResponseRowsItemBuilder::item_id)
    /// - [`min_qty`](ReorderRulesListInventoryResponseRowsItemBuilder::min_qty)
    /// - [`is_active`](ReorderRulesListInventoryResponseRowsItemBuilder::is_active)
    /// - [`created_at`](ReorderRulesListInventoryResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](ReorderRulesListInventoryResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<ReorderRulesListInventoryResponseRowsItem, BuildError> {
        Ok(ReorderRulesListInventoryResponseRowsItem {
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
