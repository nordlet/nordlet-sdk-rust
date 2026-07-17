pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsStockMovementResponseRowsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(rename = "openingQty")]
    #[serde(default)]
    pub opening_qty: String,
    #[serde(rename = "openingValue")]
    #[serde(default)]
    pub opening_value: String,
    #[serde(rename = "inQty")]
    #[serde(default)]
    pub in_qty: String,
    #[serde(rename = "inValue")]
    #[serde(default)]
    pub in_value: String,
    #[serde(rename = "outQty")]
    #[serde(default)]
    pub out_qty: String,
    #[serde(rename = "outValue")]
    #[serde(default)]
    pub out_value: String,
    #[serde(rename = "closingQty")]
    #[serde(default)]
    pub closing_qty: String,
    #[serde(rename = "closingValue")]
    #[serde(default)]
    pub closing_value: String,
}

impl PostV1ReportsStockMovementResponseRowsItem {
    pub fn builder() -> PostV1ReportsStockMovementResponseRowsItemBuilder {
        <PostV1ReportsStockMovementResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsStockMovementResponseRowsItemBuilder {
    item_id: Option<String>,
    item_name: Option<String>,
    opening_qty: Option<String>,
    opening_value: Option<String>,
    in_qty: Option<String>,
    in_value: Option<String>,
    out_qty: Option<String>,
    out_value: Option<String>,
    closing_qty: Option<String>,
    closing_value: Option<String>,
}

impl PostV1ReportsStockMovementResponseRowsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn item_name(mut self, value: impl Into<String>) -> Self {
        self.item_name = Some(value.into());
        self
    }

    pub fn opening_qty(mut self, value: impl Into<String>) -> Self {
        self.opening_qty = Some(value.into());
        self
    }

    pub fn opening_value(mut self, value: impl Into<String>) -> Self {
        self.opening_value = Some(value.into());
        self
    }

    pub fn in_qty(mut self, value: impl Into<String>) -> Self {
        self.in_qty = Some(value.into());
        self
    }

    pub fn in_value(mut self, value: impl Into<String>) -> Self {
        self.in_value = Some(value.into());
        self
    }

    pub fn out_qty(mut self, value: impl Into<String>) -> Self {
        self.out_qty = Some(value.into());
        self
    }

    pub fn out_value(mut self, value: impl Into<String>) -> Self {
        self.out_value = Some(value.into());
        self
    }

    pub fn closing_qty(mut self, value: impl Into<String>) -> Self {
        self.closing_qty = Some(value.into());
        self
    }

    pub fn closing_value(mut self, value: impl Into<String>) -> Self {
        self.closing_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsStockMovementResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1ReportsStockMovementResponseRowsItemBuilder::item_id)
    /// - [`item_name`](PostV1ReportsStockMovementResponseRowsItemBuilder::item_name)
    /// - [`opening_qty`](PostV1ReportsStockMovementResponseRowsItemBuilder::opening_qty)
    /// - [`opening_value`](PostV1ReportsStockMovementResponseRowsItemBuilder::opening_value)
    /// - [`in_qty`](PostV1ReportsStockMovementResponseRowsItemBuilder::in_qty)
    /// - [`in_value`](PostV1ReportsStockMovementResponseRowsItemBuilder::in_value)
    /// - [`out_qty`](PostV1ReportsStockMovementResponseRowsItemBuilder::out_qty)
    /// - [`out_value`](PostV1ReportsStockMovementResponseRowsItemBuilder::out_value)
    /// - [`closing_qty`](PostV1ReportsStockMovementResponseRowsItemBuilder::closing_qty)
    /// - [`closing_value`](PostV1ReportsStockMovementResponseRowsItemBuilder::closing_value)
    pub fn build(self) -> Result<PostV1ReportsStockMovementResponseRowsItem, BuildError> {
        Ok(PostV1ReportsStockMovementResponseRowsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            item_name: self
                .item_name
                .ok_or_else(|| BuildError::missing_field("item_name"))?,
            opening_qty: self
                .opening_qty
                .ok_or_else(|| BuildError::missing_field("opening_qty"))?,
            opening_value: self
                .opening_value
                .ok_or_else(|| BuildError::missing_field("opening_value"))?,
            in_qty: self
                .in_qty
                .ok_or_else(|| BuildError::missing_field("in_qty"))?,
            in_value: self
                .in_value
                .ok_or_else(|| BuildError::missing_field("in_value"))?,
            out_qty: self
                .out_qty
                .ok_or_else(|| BuildError::missing_field("out_qty"))?,
            out_value: self
                .out_value
                .ok_or_else(|| BuildError::missing_field("out_value"))?,
            closing_qty: self
                .closing_qty
                .ok_or_else(|| BuildError::missing_field("closing_qty"))?,
            closing_value: self
                .closing_value
                .ok_or_else(|| BuildError::missing_field("closing_value"))?,
        })
    }
}
