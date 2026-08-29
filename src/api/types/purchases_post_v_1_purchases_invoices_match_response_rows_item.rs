pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesInvoicesMatchResponseRowsItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "orderedQty")]
    #[serde(default)]
    pub ordered_qty: String,
    #[serde(rename = "receivedQty")]
    #[serde(default)]
    pub received_qty: String,
    #[serde(rename = "invoicedQty")]
    #[serde(default)]
    pub invoiced_qty: String,
    #[serde(rename = "orderedUnitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered_unit_price: Option<String>,
    #[serde(rename = "invoicedUnitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiced_unit_price: Option<String>,
    #[serde(rename = "priceVariancePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_variance_percent: Option<String>,
    pub status: PostV1PurchasesInvoicesMatchResponseRowsItemStatus,
}

impl PostV1PurchasesInvoicesMatchResponseRowsItem {
    pub fn builder() -> PostV1PurchasesInvoicesMatchResponseRowsItemBuilder {
        <PostV1PurchasesInvoicesMatchResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesMatchResponseRowsItemBuilder {
    item_id: Option<String>,
    description: Option<String>,
    ordered_qty: Option<String>,
    received_qty: Option<String>,
    invoiced_qty: Option<String>,
    ordered_unit_price: Option<String>,
    invoiced_unit_price: Option<String>,
    price_variance_percent: Option<String>,
    status: Option<PostV1PurchasesInvoicesMatchResponseRowsItemStatus>,
}

impl PostV1PurchasesInvoicesMatchResponseRowsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn ordered_qty(mut self, value: impl Into<String>) -> Self {
        self.ordered_qty = Some(value.into());
        self
    }

    pub fn received_qty(mut self, value: impl Into<String>) -> Self {
        self.received_qty = Some(value.into());
        self
    }

    pub fn invoiced_qty(mut self, value: impl Into<String>) -> Self {
        self.invoiced_qty = Some(value.into());
        self
    }

    pub fn ordered_unit_price(mut self, value: impl Into<String>) -> Self {
        self.ordered_unit_price = Some(value.into());
        self
    }

    pub fn invoiced_unit_price(mut self, value: impl Into<String>) -> Self {
        self.invoiced_unit_price = Some(value.into());
        self
    }

    pub fn price_variance_percent(mut self, value: impl Into<String>) -> Self {
        self.price_variance_percent = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1PurchasesInvoicesMatchResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesMatchResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](PostV1PurchasesInvoicesMatchResponseRowsItemBuilder::description)
    /// - [`ordered_qty`](PostV1PurchasesInvoicesMatchResponseRowsItemBuilder::ordered_qty)
    /// - [`received_qty`](PostV1PurchasesInvoicesMatchResponseRowsItemBuilder::received_qty)
    /// - [`invoiced_qty`](PostV1PurchasesInvoicesMatchResponseRowsItemBuilder::invoiced_qty)
    /// - [`status`](PostV1PurchasesInvoicesMatchResponseRowsItemBuilder::status)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesMatchResponseRowsItem, BuildError> {
        Ok(PostV1PurchasesInvoicesMatchResponseRowsItem {
            item_id: self.item_id,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            ordered_qty: self
                .ordered_qty
                .ok_or_else(|| BuildError::missing_field("ordered_qty"))?,
            received_qty: self
                .received_qty
                .ok_or_else(|| BuildError::missing_field("received_qty"))?,
            invoiced_qty: self
                .invoiced_qty
                .ok_or_else(|| BuildError::missing_field("invoiced_qty"))?,
            ordered_unit_price: self.ordered_unit_price,
            invoiced_unit_price: self.invoiced_unit_price,
            price_variance_percent: self.price_variance_percent,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
