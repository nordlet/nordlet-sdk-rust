pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsStockShortageResponseRowsItem {
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

impl PostV1ReportsStockShortageResponseRowsItem {
    pub fn builder() -> PostV1ReportsStockShortageResponseRowsItemBuilder {
        <PostV1ReportsStockShortageResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsStockShortageResponseRowsItemBuilder {
    item_id: Option<String>,
    item_name: Option<String>,
    warehouse_id: Option<String>,
    on_hand: Option<String>,
    reserved: Option<String>,
    shortage: Option<String>,
}

impl PostV1ReportsStockShortageResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsStockShortageResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1ReportsStockShortageResponseRowsItemBuilder::item_id)
    /// - [`item_name`](PostV1ReportsStockShortageResponseRowsItemBuilder::item_name)
    /// - [`warehouse_id`](PostV1ReportsStockShortageResponseRowsItemBuilder::warehouse_id)
    /// - [`on_hand`](PostV1ReportsStockShortageResponseRowsItemBuilder::on_hand)
    /// - [`reserved`](PostV1ReportsStockShortageResponseRowsItemBuilder::reserved)
    /// - [`shortage`](PostV1ReportsStockShortageResponseRowsItemBuilder::shortage)
    pub fn build(self) -> Result<PostV1ReportsStockShortageResponseRowsItem, BuildError> {
        Ok(PostV1ReportsStockShortageResponseRowsItem {
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
