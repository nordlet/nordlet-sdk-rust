pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockTakeInventoryRequestLinesItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(rename = "countedQty")]
    #[serde(default)]
    pub counted_qty: String,
    #[serde(rename = "unitCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<String>,
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
}

impl StockTakeInventoryRequestLinesItem {
    pub fn builder() -> StockTakeInventoryRequestLinesItemBuilder {
        <StockTakeInventoryRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockTakeInventoryRequestLinesItemBuilder {
    item_id: Option<String>,
    barcode: Option<String>,
    counted_qty: Option<String>,
    unit_cost: Option<String>,
    lot_number: Option<String>,
    expiry_date: Option<NaiveDate>,
}

impl StockTakeInventoryRequestLinesItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn barcode(mut self, value: impl Into<String>) -> Self {
        self.barcode = Some(value.into());
        self
    }

    pub fn counted_qty(mut self, value: impl Into<String>) -> Self {
        self.counted_qty = Some(value.into());
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

    /// Consumes the builder and constructs a [`StockTakeInventoryRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`counted_qty`](StockTakeInventoryRequestLinesItemBuilder::counted_qty)
    pub fn build(self) -> Result<StockTakeInventoryRequestLinesItem, BuildError> {
        Ok(StockTakeInventoryRequestLinesItem {
            item_id: self.item_id,
            barcode: self.barcode,
            counted_qty: self
                .counted_qty
                .ok_or_else(|| BuildError::missing_field("counted_qty"))?,
            unit_cost: self.unit_cost,
            lot_number: self.lot_number,
            expiry_date: self.expiry_date,
        })
    }
}
