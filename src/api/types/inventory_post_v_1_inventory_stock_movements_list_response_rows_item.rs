pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockMovementsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub date: String,
    pub direction: PostV1InventoryStockMovementsListResponseRowsItemDirection,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<String>,
    #[serde(rename = "totalCost")]
    #[serde(default)]
    pub total_cost: String,
    #[serde(rename = "remainingQty")]
    #[serde(default)]
    pub remaining_qty: String,
    #[serde(rename = "documentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "documentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1InventoryStockMovementsListResponseRowsItem {
    pub fn builder() -> PostV1InventoryStockMovementsListResponseRowsItemBuilder {
        <PostV1InventoryStockMovementsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockMovementsListResponseRowsItemBuilder {
    id: Option<String>,
    warehouse_id: Option<String>,
    item_id: Option<String>,
    date: Option<String>,
    direction: Option<PostV1InventoryStockMovementsListResponseRowsItemDirection>,
    quantity: Option<String>,
    unit_cost: Option<String>,
    total_cost: Option<String>,
    remaining_qty: Option<String>,
    document_type: Option<String>,
    document_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1InventoryStockMovementsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn direction(
        mut self,
        value: PostV1InventoryStockMovementsListResponseRowsItemDirection,
    ) -> Self {
        self.direction = Some(value);
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

    pub fn total_cost(mut self, value: impl Into<String>) -> Self {
        self.total_cost = Some(value.into());
        self
    }

    pub fn remaining_qty(mut self, value: impl Into<String>) -> Self {
        self.remaining_qty = Some(value.into());
        self
    }

    pub fn document_type(mut self, value: impl Into<String>) -> Self {
        self.document_type = Some(value.into());
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1InventoryStockMovementsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryStockMovementsListResponseRowsItemBuilder::id)
    /// - [`warehouse_id`](PostV1InventoryStockMovementsListResponseRowsItemBuilder::warehouse_id)
    /// - [`item_id`](PostV1InventoryStockMovementsListResponseRowsItemBuilder::item_id)
    /// - [`date`](PostV1InventoryStockMovementsListResponseRowsItemBuilder::date)
    /// - [`direction`](PostV1InventoryStockMovementsListResponseRowsItemBuilder::direction)
    /// - [`quantity`](PostV1InventoryStockMovementsListResponseRowsItemBuilder::quantity)
    /// - [`total_cost`](PostV1InventoryStockMovementsListResponseRowsItemBuilder::total_cost)
    /// - [`remaining_qty`](PostV1InventoryStockMovementsListResponseRowsItemBuilder::remaining_qty)
    /// - [`created_at`](PostV1InventoryStockMovementsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1InventoryStockMovementsListResponseRowsItem, BuildError> {
        Ok(PostV1InventoryStockMovementsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_cost: self.unit_cost,
            total_cost: self
                .total_cost
                .ok_or_else(|| BuildError::missing_field("total_cost"))?,
            remaining_qty: self
                .remaining_qty
                .ok_or_else(|| BuildError::missing_field("remaining_qty"))?,
            document_type: self.document_type,
            document_id: self.document_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
