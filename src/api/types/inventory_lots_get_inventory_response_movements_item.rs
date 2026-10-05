pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LotsGetInventoryResponseMovementsItem {
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
    pub date: NaiveDate,
    pub direction: LotsGetInventoryResponseMovementsItemDirection,
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl LotsGetInventoryResponseMovementsItem {
    pub fn builder() -> LotsGetInventoryResponseMovementsItemBuilder {
        <LotsGetInventoryResponseMovementsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LotsGetInventoryResponseMovementsItemBuilder {
    id: Option<String>,
    warehouse_id: Option<String>,
    item_id: Option<String>,
    lot_id: Option<String>,
    date: Option<NaiveDate>,
    direction: Option<LotsGetInventoryResponseMovementsItemDirection>,
    quantity: Option<String>,
    unit_cost: Option<String>,
    total_cost: Option<String>,
    remaining_qty: Option<String>,
    document_type: Option<String>,
    document_id: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl LotsGetInventoryResponseMovementsItemBuilder {
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

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn direction(mut self, value: LotsGetInventoryResponseMovementsItemDirection) -> Self {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LotsGetInventoryResponseMovementsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LotsGetInventoryResponseMovementsItemBuilder::id)
    /// - [`warehouse_id`](LotsGetInventoryResponseMovementsItemBuilder::warehouse_id)
    /// - [`item_id`](LotsGetInventoryResponseMovementsItemBuilder::item_id)
    /// - [`date`](LotsGetInventoryResponseMovementsItemBuilder::date)
    /// - [`direction`](LotsGetInventoryResponseMovementsItemBuilder::direction)
    /// - [`quantity`](LotsGetInventoryResponseMovementsItemBuilder::quantity)
    /// - [`total_cost`](LotsGetInventoryResponseMovementsItemBuilder::total_cost)
    /// - [`remaining_qty`](LotsGetInventoryResponseMovementsItemBuilder::remaining_qty)
    /// - [`created_at`](LotsGetInventoryResponseMovementsItemBuilder::created_at)
    pub fn build(self) -> Result<LotsGetInventoryResponseMovementsItem, BuildError> {
        Ok(LotsGetInventoryResponseMovementsItem {
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
