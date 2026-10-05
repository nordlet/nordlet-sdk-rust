pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsCreatePurchasesRequestLinesItem {
    #[serde(rename = "orderLineId")]
    #[serde(default)]
    pub order_line_id: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
}

impl ReceiptsCreatePurchasesRequestLinesItem {
    pub fn builder() -> ReceiptsCreatePurchasesRequestLinesItemBuilder {
        <ReceiptsCreatePurchasesRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsCreatePurchasesRequestLinesItemBuilder {
    order_line_id: Option<String>,
    quantity: Option<String>,
    lot_number: Option<String>,
    expiry_date: Option<NaiveDate>,
}

impl ReceiptsCreatePurchasesRequestLinesItemBuilder {
    pub fn order_line_id(mut self, value: impl Into<String>) -> Self {
        self.order_line_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn expiry_date(mut self, value: NaiveDate) -> Self {
        self.expiry_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsCreatePurchasesRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_line_id`](ReceiptsCreatePurchasesRequestLinesItemBuilder::order_line_id)
    /// - [`quantity`](ReceiptsCreatePurchasesRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<ReceiptsCreatePurchasesRequestLinesItem, BuildError> {
        Ok(ReceiptsCreatePurchasesRequestLinesItem {
            order_line_id: self
                .order_line_id
                .ok_or_else(|| BuildError::missing_field("order_line_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            lot_number: self.lot_number,
            expiry_date: self.expiry_date,
        })
    }
}
