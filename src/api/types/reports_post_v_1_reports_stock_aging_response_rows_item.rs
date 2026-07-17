pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsStockAgingResponseRowsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(rename = "d0to30Qty")]
    #[serde(default)]
    pub d0to30_qty: String,
    #[serde(rename = "d0to30Value")]
    #[serde(default)]
    pub d0to30_value: String,
    #[serde(rename = "d31to60Qty")]
    #[serde(default)]
    pub d31to60_qty: String,
    #[serde(rename = "d31to60Value")]
    #[serde(default)]
    pub d31to60_value: String,
    #[serde(rename = "d61to90Qty")]
    #[serde(default)]
    pub d61to90_qty: String,
    #[serde(rename = "d61to90Value")]
    #[serde(default)]
    pub d61to90_value: String,
    #[serde(rename = "over90Qty")]
    #[serde(default)]
    pub over90_qty: String,
    #[serde(rename = "over90Value")]
    #[serde(default)]
    pub over90_value: String,
    #[serde(rename = "totalQty")]
    #[serde(default)]
    pub total_qty: String,
    #[serde(rename = "totalValue")]
    #[serde(default)]
    pub total_value: String,
}

impl PostV1ReportsStockAgingResponseRowsItem {
    pub fn builder() -> PostV1ReportsStockAgingResponseRowsItemBuilder {
        <PostV1ReportsStockAgingResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsStockAgingResponseRowsItemBuilder {
    item_id: Option<String>,
    item_name: Option<String>,
    warehouse_id: Option<String>,
    d0to30_qty: Option<String>,
    d0to30_value: Option<String>,
    d31to60_qty: Option<String>,
    d31to60_value: Option<String>,
    d61to90_qty: Option<String>,
    d61to90_value: Option<String>,
    over90_qty: Option<String>,
    over90_value: Option<String>,
    total_qty: Option<String>,
    total_value: Option<String>,
}

impl PostV1ReportsStockAgingResponseRowsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn item_name(mut self, value: impl Into<String>) -> Self {
        self.item_name = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn d0to30_qty(mut self, value: impl Into<String>) -> Self {
        self.d0to30_qty = Some(value.into());
        self
    }

    pub fn d0to30_value(mut self, value: impl Into<String>) -> Self {
        self.d0to30_value = Some(value.into());
        self
    }

    pub fn d31to60_qty(mut self, value: impl Into<String>) -> Self {
        self.d31to60_qty = Some(value.into());
        self
    }

    pub fn d31to60_value(mut self, value: impl Into<String>) -> Self {
        self.d31to60_value = Some(value.into());
        self
    }

    pub fn d61to90_qty(mut self, value: impl Into<String>) -> Self {
        self.d61to90_qty = Some(value.into());
        self
    }

    pub fn d61to90_value(mut self, value: impl Into<String>) -> Self {
        self.d61to90_value = Some(value.into());
        self
    }

    pub fn over90_qty(mut self, value: impl Into<String>) -> Self {
        self.over90_qty = Some(value.into());
        self
    }

    pub fn over90_value(mut self, value: impl Into<String>) -> Self {
        self.over90_value = Some(value.into());
        self
    }

    pub fn total_qty(mut self, value: impl Into<String>) -> Self {
        self.total_qty = Some(value.into());
        self
    }

    pub fn total_value(mut self, value: impl Into<String>) -> Self {
        self.total_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsStockAgingResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1ReportsStockAgingResponseRowsItemBuilder::item_id)
    /// - [`item_name`](PostV1ReportsStockAgingResponseRowsItemBuilder::item_name)
    /// - [`warehouse_id`](PostV1ReportsStockAgingResponseRowsItemBuilder::warehouse_id)
    /// - [`d0to30_qty`](PostV1ReportsStockAgingResponseRowsItemBuilder::d0to30_qty)
    /// - [`d0to30_value`](PostV1ReportsStockAgingResponseRowsItemBuilder::d0to30_value)
    /// - [`d31to60_qty`](PostV1ReportsStockAgingResponseRowsItemBuilder::d31to60_qty)
    /// - [`d31to60_value`](PostV1ReportsStockAgingResponseRowsItemBuilder::d31to60_value)
    /// - [`d61to90_qty`](PostV1ReportsStockAgingResponseRowsItemBuilder::d61to90_qty)
    /// - [`d61to90_value`](PostV1ReportsStockAgingResponseRowsItemBuilder::d61to90_value)
    /// - [`over90_qty`](PostV1ReportsStockAgingResponseRowsItemBuilder::over90_qty)
    /// - [`over90_value`](PostV1ReportsStockAgingResponseRowsItemBuilder::over90_value)
    /// - [`total_qty`](PostV1ReportsStockAgingResponseRowsItemBuilder::total_qty)
    /// - [`total_value`](PostV1ReportsStockAgingResponseRowsItemBuilder::total_value)
    pub fn build(self) -> Result<PostV1ReportsStockAgingResponseRowsItem, BuildError> {
        Ok(PostV1ReportsStockAgingResponseRowsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            item_name: self
                .item_name
                .ok_or_else(|| BuildError::missing_field("item_name"))?,
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            d0to30_qty: self
                .d0to30_qty
                .ok_or_else(|| BuildError::missing_field("d0to30_qty"))?,
            d0to30_value: self
                .d0to30_value
                .ok_or_else(|| BuildError::missing_field("d0to30_value"))?,
            d31to60_qty: self
                .d31to60_qty
                .ok_or_else(|| BuildError::missing_field("d31to60_qty"))?,
            d31to60_value: self
                .d31to60_value
                .ok_or_else(|| BuildError::missing_field("d31to60_value"))?,
            d61to90_qty: self
                .d61to90_qty
                .ok_or_else(|| BuildError::missing_field("d61to90_qty"))?,
            d61to90_value: self
                .d61to90_value
                .ok_or_else(|| BuildError::missing_field("d61to90_value"))?,
            over90_qty: self
                .over90_qty
                .ok_or_else(|| BuildError::missing_field("over90_qty"))?,
            over90_value: self
                .over90_value
                .ok_or_else(|| BuildError::missing_field("over90_value"))?,
            total_qty: self
                .total_qty
                .ok_or_else(|| BuildError::missing_field("total_qty"))?,
            total_value: self
                .total_value
                .ok_or_else(|| BuildError::missing_field("total_value"))?,
        })
    }
}
