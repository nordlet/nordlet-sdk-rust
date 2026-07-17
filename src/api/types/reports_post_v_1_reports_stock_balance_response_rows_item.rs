pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsStockBalanceResponseRowsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1ReportsStockBalanceResponseRowsItem {
    pub fn builder() -> PostV1ReportsStockBalanceResponseRowsItemBuilder {
        <PostV1ReportsStockBalanceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsStockBalanceResponseRowsItemBuilder {
    item_id: Option<String>,
    item_name: Option<String>,
    warehouse_id: Option<String>,
    quantity: Option<String>,
    value: Option<String>,
}

impl PostV1ReportsStockBalanceResponseRowsItemBuilder {
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

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsStockBalanceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1ReportsStockBalanceResponseRowsItemBuilder::item_id)
    /// - [`item_name`](PostV1ReportsStockBalanceResponseRowsItemBuilder::item_name)
    /// - [`warehouse_id`](PostV1ReportsStockBalanceResponseRowsItemBuilder::warehouse_id)
    /// - [`quantity`](PostV1ReportsStockBalanceResponseRowsItemBuilder::quantity)
    /// - [`value`](PostV1ReportsStockBalanceResponseRowsItemBuilder::value)
    pub fn build(self) -> Result<PostV1ReportsStockBalanceResponseRowsItem, BuildError> {
        Ok(PostV1ReportsStockBalanceResponseRowsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            item_name: self
                .item_name
                .ok_or_else(|| BuildError::missing_field("item_name"))?,
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
