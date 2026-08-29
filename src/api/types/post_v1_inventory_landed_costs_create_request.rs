pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLandedCostsCreateRequest {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PostV1InventoryLandedCostsCreateRequestMethod>,
    #[serde(rename = "goodsReceiptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_receipt_id: Option<String>,
    #[serde(rename = "movementIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_ids: Option<Vec<String>>,
    #[serde(rename = "sourceInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1InventoryLandedCostsCreateRequest {
    pub fn builder() -> PostV1InventoryLandedCostsCreateRequestBuilder {
        <PostV1InventoryLandedCostsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLandedCostsCreateRequestBuilder {
    date: Option<String>,
    amount: Option<String>,
    method: Option<PostV1InventoryLandedCostsCreateRequestMethod>,
    goods_receipt_id: Option<String>,
    movement_ids: Option<Vec<String>>,
    source_invoice_id: Option<String>,
    notes: Option<String>,
}

impl PostV1InventoryLandedCostsCreateRequestBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn method(mut self, value: PostV1InventoryLandedCostsCreateRequestMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn goods_receipt_id(mut self, value: impl Into<String>) -> Self {
        self.goods_receipt_id = Some(value.into());
        self
    }

    pub fn movement_ids(mut self, value: Vec<String>) -> Self {
        self.movement_ids = Some(value);
        self
    }

    pub fn source_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.source_invoice_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLandedCostsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1InventoryLandedCostsCreateRequestBuilder::date)
    /// - [`amount`](PostV1InventoryLandedCostsCreateRequestBuilder::amount)
    pub fn build(self) -> Result<PostV1InventoryLandedCostsCreateRequest, BuildError> {
        Ok(PostV1InventoryLandedCostsCreateRequest {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            method: self.method,
            goods_receipt_id: self.goods_receipt_id,
            movement_ids: self.movement_ids,
            source_invoice_id: self.source_invoice_id,
            notes: self.notes,
        })
    }
}
