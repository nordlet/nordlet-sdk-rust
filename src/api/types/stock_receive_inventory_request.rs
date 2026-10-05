pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockReceiveInventoryRequest {
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "unitCost")]
    #[serde(default)]
    pub unit_cost: String,
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl StockReceiveInventoryRequest {
    pub fn builder() -> StockReceiveInventoryRequestBuilder {
        <StockReceiveInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockReceiveInventoryRequestBuilder {
    warehouse_id: Option<String>,
    item_id: Option<String>,
    date: Option<NaiveDate>,
    quantity: Option<String>,
    unit_cost: Option<String>,
    lot_number: Option<String>,
    expiry_date: Option<NaiveDate>,
    notes: Option<String>,
}

impl StockReceiveInventoryRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
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

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn expiry_date(mut self, value: NaiveDate) -> Self {
        self.expiry_date = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockReceiveInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`warehouse_id`](StockReceiveInventoryRequestBuilder::warehouse_id)
    /// - [`item_id`](StockReceiveInventoryRequestBuilder::item_id)
    /// - [`date`](StockReceiveInventoryRequestBuilder::date)
    /// - [`quantity`](StockReceiveInventoryRequestBuilder::quantity)
    /// - [`unit_cost`](StockReceiveInventoryRequestBuilder::unit_cost)
    pub fn build(self) -> Result<StockReceiveInventoryRequest, BuildError> {
        Ok(StockReceiveInventoryRequest {
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_cost: self
                .unit_cost
                .ok_or_else(|| BuildError::missing_field("unit_cost"))?,
            lot_number: self.lot_number,
            expiry_date: self.expiry_date,
            notes: self.notes,
        })
    }
}
