pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLotsGetResponseMovementsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "lotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_id: Option<String>,
    #[serde(default)]
    pub date: String,
    pub direction: PostV1InventoryLotsGetResponseMovementsItemDirection,
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

impl PostV1InventoryLotsGetResponseMovementsItem {
    pub fn builder() -> PostV1InventoryLotsGetResponseMovementsItemBuilder {
        <PostV1InventoryLotsGetResponseMovementsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLotsGetResponseMovementsItemBuilder {
    id: Option<String>,
    warehouse_id: Option<String>,
    item_id: Option<String>,
    lot_id: Option<String>,
    date: Option<String>,
    direction: Option<PostV1InventoryLotsGetResponseMovementsItemDirection>,
    quantity: Option<String>,
    unit_cost: Option<String>,
    total_cost: Option<String>,
    remaining_qty: Option<String>,
    document_type: Option<String>,
    document_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1InventoryLotsGetResponseMovementsItemBuilder {
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

    pub fn lot_id(mut self, value: impl Into<String>) -> Self {
        self.lot_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn direction(
        mut self,
        value: PostV1InventoryLotsGetResponseMovementsItemDirection,
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

    /// Consumes the builder and constructs a [`PostV1InventoryLotsGetResponseMovementsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryLotsGetResponseMovementsItemBuilder::id)
    /// - [`warehouse_id`](PostV1InventoryLotsGetResponseMovementsItemBuilder::warehouse_id)
    /// - [`item_id`](PostV1InventoryLotsGetResponseMovementsItemBuilder::item_id)
    /// - [`date`](PostV1InventoryLotsGetResponseMovementsItemBuilder::date)
    /// - [`direction`](PostV1InventoryLotsGetResponseMovementsItemBuilder::direction)
    /// - [`quantity`](PostV1InventoryLotsGetResponseMovementsItemBuilder::quantity)
    /// - [`total_cost`](PostV1InventoryLotsGetResponseMovementsItemBuilder::total_cost)
    /// - [`remaining_qty`](PostV1InventoryLotsGetResponseMovementsItemBuilder::remaining_qty)
    /// - [`created_at`](PostV1InventoryLotsGetResponseMovementsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1InventoryLotsGetResponseMovementsItem, BuildError> {
        Ok(PostV1InventoryLotsGetResponseMovementsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            lot_id: self.lot_id,
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
