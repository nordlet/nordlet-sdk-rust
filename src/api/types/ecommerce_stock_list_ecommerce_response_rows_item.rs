pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockListEcommerceResponseRowsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(rename = "onHand")]
    #[serde(default)]
    pub on_hand: String,
    #[serde(default)]
    pub reserved: String,
    #[serde(default)]
    pub available: String,
}

impl StockListEcommerceResponseRowsItem {
    pub fn builder() -> StockListEcommerceResponseRowsItemBuilder {
        <StockListEcommerceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockListEcommerceResponseRowsItemBuilder {
    item_id: Option<String>,
    warehouse_id: Option<String>,
    on_hand: Option<String>,
    reserved: Option<String>,
    available: Option<String>,
}

impl StockListEcommerceResponseRowsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
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

    pub fn available(mut self, value: impl Into<String>) -> Self {
        self.available = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockListEcommerceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](StockListEcommerceResponseRowsItemBuilder::item_id)
    /// - [`warehouse_id`](StockListEcommerceResponseRowsItemBuilder::warehouse_id)
    /// - [`on_hand`](StockListEcommerceResponseRowsItemBuilder::on_hand)
    /// - [`reserved`](StockListEcommerceResponseRowsItemBuilder::reserved)
    /// - [`available`](StockListEcommerceResponseRowsItemBuilder::available)
    pub fn build(self) -> Result<StockListEcommerceResponseRowsItem, BuildError> {
        Ok(StockListEcommerceResponseRowsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            on_hand: self
                .on_hand
                .ok_or_else(|| BuildError::missing_field("on_hand"))?,
            reserved: self
                .reserved
                .ok_or_else(|| BuildError::missing_field("reserved"))?,
            available: self
                .available
                .ok_or_else(|| BuildError::missing_field("available"))?,
        })
    }
}
