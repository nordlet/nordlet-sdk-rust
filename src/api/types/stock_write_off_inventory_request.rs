pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockWriteOffInventoryRequest {
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
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(rename = "expenseAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_code: Option<String>,
    #[serde(rename = "inventoryAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl StockWriteOffInventoryRequest {
    pub fn builder() -> StockWriteOffInventoryRequestBuilder {
        <StockWriteOffInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockWriteOffInventoryRequestBuilder {
    warehouse_id: Option<String>,
    item_id: Option<String>,
    date: Option<NaiveDate>,
    quantity: Option<String>,
    lot_number: Option<String>,
    expense_account_code: Option<String>,
    inventory_account_code: Option<String>,
    notes: Option<String>,
}

impl StockWriteOffInventoryRequestBuilder {
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

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn expense_account_code(mut self, value: impl Into<String>) -> Self {
        self.expense_account_code = Some(value.into());
        self
    }

    pub fn inventory_account_code(mut self, value: impl Into<String>) -> Self {
        self.inventory_account_code = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockWriteOffInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`warehouse_id`](StockWriteOffInventoryRequestBuilder::warehouse_id)
    /// - [`item_id`](StockWriteOffInventoryRequestBuilder::item_id)
    /// - [`date`](StockWriteOffInventoryRequestBuilder::date)
    /// - [`quantity`](StockWriteOffInventoryRequestBuilder::quantity)
    pub fn build(self) -> Result<StockWriteOffInventoryRequest, BuildError> {
        Ok(StockWriteOffInventoryRequest {
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
            lot_number: self.lot_number,
            expense_account_code: self.expense_account_code,
            inventory_account_code: self.inventory_account_code,
            notes: self.notes,
        })
    }
}
