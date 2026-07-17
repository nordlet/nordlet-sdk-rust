pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockTakeRequest {
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(default)]
    pub date: String,
    #[serde(rename = "expenseAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_code: Option<String>,
    #[serde(rename = "inventoryAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_account_code: Option<String>,
    #[serde(default)]
    pub lines: Vec<PostV1InventoryStockTakeRequestLinesItem>,
}

impl PostV1InventoryStockTakeRequest {
    pub fn builder() -> PostV1InventoryStockTakeRequestBuilder {
        <PostV1InventoryStockTakeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockTakeRequestBuilder {
    warehouse_id: Option<String>,
    date: Option<String>,
    expense_account_code: Option<String>,
    inventory_account_code: Option<String>,
    lines: Option<Vec<PostV1InventoryStockTakeRequestLinesItem>>,
}

impl PostV1InventoryStockTakeRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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

    pub fn lines(mut self, value: Vec<PostV1InventoryStockTakeRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockTakeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`warehouse_id`](PostV1InventoryStockTakeRequestBuilder::warehouse_id)
    /// - [`date`](PostV1InventoryStockTakeRequestBuilder::date)
    /// - [`lines`](PostV1InventoryStockTakeRequestBuilder::lines)
    pub fn build(self) -> Result<PostV1InventoryStockTakeRequest, BuildError> {
        Ok(PostV1InventoryStockTakeRequest {
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            expense_account_code: self.expense_account_code,
            inventory_account_code: self.inventory_account_code,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
