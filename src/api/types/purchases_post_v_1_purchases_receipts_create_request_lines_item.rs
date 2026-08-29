pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesReceiptsCreateRequestLinesItem {
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
    pub expiry_date: Option<String>,
}

impl PostV1PurchasesReceiptsCreateRequestLinesItem {
    pub fn builder() -> PostV1PurchasesReceiptsCreateRequestLinesItemBuilder {
        <PostV1PurchasesReceiptsCreateRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesReceiptsCreateRequestLinesItemBuilder {
    order_line_id: Option<String>,
    quantity: Option<String>,
    lot_number: Option<String>,
    expiry_date: Option<String>,
}

impl PostV1PurchasesReceiptsCreateRequestLinesItemBuilder {
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

    pub fn expiry_date(mut self, value: impl Into<String>) -> Self {
        self.expiry_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesReceiptsCreateRequestLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_line_id`](PostV1PurchasesReceiptsCreateRequestLinesItemBuilder::order_line_id)
    /// - [`quantity`](PostV1PurchasesReceiptsCreateRequestLinesItemBuilder::quantity)
    pub fn build(self) -> Result<PostV1PurchasesReceiptsCreateRequestLinesItem, BuildError> {
        Ok(PostV1PurchasesReceiptsCreateRequestLinesItem {
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
