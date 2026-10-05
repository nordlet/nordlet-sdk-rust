pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockShortageReportsResponseRowsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(rename = "onHand")]
    #[serde(default)]
    pub on_hand: String,
    #[serde(default)]
    pub reserved: String,
    #[serde(default)]
    pub shortage: String,
}

impl StockShortageReportsResponseRowsItem {
    pub fn builder() -> StockShortageReportsResponseRowsItemBuilder {
        <StockShortageReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockShortageReportsResponseRowsItemBuilder {
    item_id: Option<String>,
    item_name: Option<String>,
    warehouse_id: Option<String>,
    on_hand: Option<String>,
    reserved: Option<String>,
    shortage: Option<String>,
}

impl StockShortageReportsResponseRowsItemBuilder {
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

    pub fn on_hand(mut self, value: impl Into<String>) -> Self {
        self.on_hand = Some(value.into());
        self
    }

    pub fn reserved(mut self, value: impl Into<String>) -> Self {
        self.reserved = Some(value.into());
        self
    }

    pub fn shortage(mut self, value: impl Into<String>) -> Self {
        self.shortage = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockShortageReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](StockShortageReportsResponseRowsItemBuilder::item_id)
    /// - [`item_name`](StockShortageReportsResponseRowsItemBuilder::item_name)
    /// - [`warehouse_id`](StockShortageReportsResponseRowsItemBuilder::warehouse_id)
    /// - [`on_hand`](StockShortageReportsResponseRowsItemBuilder::on_hand)
    /// - [`reserved`](StockShortageReportsResponseRowsItemBuilder::reserved)
    /// - [`shortage`](StockShortageReportsResponseRowsItemBuilder::shortage)
    pub fn build(self) -> Result<StockShortageReportsResponseRowsItem, BuildError> {
        Ok(StockShortageReportsResponseRowsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            item_name: self
                .item_name
                .ok_or_else(|| BuildError::missing_field("item_name"))?,
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            on_hand: self
                .on_hand
                .ok_or_else(|| BuildError::missing_field("on_hand"))?,
            reserved: self
                .reserved
                .ok_or_else(|| BuildError::missing_field("reserved"))?,
            shortage: self
                .shortage
                .ok_or_else(|| BuildError::missing_field("shortage"))?,
        })
    }
}
