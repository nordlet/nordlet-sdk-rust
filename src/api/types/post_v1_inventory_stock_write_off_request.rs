pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockWriteOffRequest {
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "expenseAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_code: Option<String>,
    #[serde(rename = "inventoryAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1InventoryStockWriteOffRequest {
    pub fn builder() -> PostV1InventoryStockWriteOffRequestBuilder {
        <PostV1InventoryStockWriteOffRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockWriteOffRequestBuilder {
    warehouse_id: Option<String>,
    item_id: Option<String>,
    date: Option<String>,
    quantity: Option<String>,
    expense_account_code: Option<String>,
    inventory_account_code: Option<String>,
    notes: Option<String>,
}

impl PostV1InventoryStockWriteOffRequestBuilder {
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

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1InventoryStockWriteOffRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`warehouse_id`](PostV1InventoryStockWriteOffRequestBuilder::warehouse_id)
    /// - [`item_id`](PostV1InventoryStockWriteOffRequestBuilder::item_id)
    /// - [`date`](PostV1InventoryStockWriteOffRequestBuilder::date)
    /// - [`quantity`](PostV1InventoryStockWriteOffRequestBuilder::quantity)
    pub fn build(self) -> Result<PostV1InventoryStockWriteOffRequest, BuildError> {
        Ok(PostV1InventoryStockWriteOffRequest {
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
            expense_account_code: self.expense_account_code,
            inventory_account_code: self.inventory_account_code,
            notes: self.notes,
        })
    }
}
