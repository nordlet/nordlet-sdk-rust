pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockTakeInventoryRequest {
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "expenseAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_code: Option<String>,
    #[serde(rename = "inventoryAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_account_code: Option<String>,
    #[serde(default)]
    pub lines: Vec<StockTakeInventoryRequestLinesItem>,
}

impl StockTakeInventoryRequest {
    pub fn builder() -> StockTakeInventoryRequestBuilder {
        <StockTakeInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockTakeInventoryRequestBuilder {
    warehouse_id: Option<String>,
    date: Option<NaiveDate>,
    expense_account_code: Option<String>,
    inventory_account_code: Option<String>,
    lines: Option<Vec<StockTakeInventoryRequestLinesItem>>,
}

impl StockTakeInventoryRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
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

    pub fn lines(mut self, value: Vec<StockTakeInventoryRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StockTakeInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`warehouse_id`](StockTakeInventoryRequestBuilder::warehouse_id)
    /// - [`date`](StockTakeInventoryRequestBuilder::date)
    /// - [`lines`](StockTakeInventoryRequestBuilder::lines)
    pub fn build(self) -> Result<StockTakeInventoryRequest, BuildError> {
        Ok(StockTakeInventoryRequest {
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
