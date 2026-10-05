pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockLevelsInventoryResponseRowsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(default)]
    pub value: String,
}

impl StockLevelsInventoryResponseRowsItem {
    pub fn builder() -> StockLevelsInventoryResponseRowsItemBuilder {
        <StockLevelsInventoryResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockLevelsInventoryResponseRowsItemBuilder {
    item_id: Option<String>,
    warehouse_id: Option<String>,
    quantity: Option<String>,
    value: Option<String>,
}

impl StockLevelsInventoryResponseRowsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`StockLevelsInventoryResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](StockLevelsInventoryResponseRowsItemBuilder::item_id)
    /// - [`warehouse_id`](StockLevelsInventoryResponseRowsItemBuilder::warehouse_id)
    /// - [`quantity`](StockLevelsInventoryResponseRowsItemBuilder::quantity)
    /// - [`value`](StockLevelsInventoryResponseRowsItemBuilder::value)
    pub fn build(self) -> Result<StockLevelsInventoryResponseRowsItem, BuildError> {
        Ok(StockLevelsInventoryResponseRowsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
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
