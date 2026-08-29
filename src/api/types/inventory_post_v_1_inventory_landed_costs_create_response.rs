pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLandedCostsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub amount: String,
    pub method: PostV1InventoryLandedCostsCreateResponseMethod,
    #[serde(rename = "goodsReceiptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_receipt_id: Option<String>,
    #[serde(rename = "sourceInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub lines: Vec<PostV1InventoryLandedCostsCreateResponseLinesItem>,
}

impl PostV1InventoryLandedCostsCreateResponse {
    pub fn builder() -> PostV1InventoryLandedCostsCreateResponseBuilder {
        <PostV1InventoryLandedCostsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLandedCostsCreateResponseBuilder {
    id: Option<String>,
    date: Option<String>,
    amount: Option<String>,
    method: Option<PostV1InventoryLandedCostsCreateResponseMethod>,
    goods_receipt_id: Option<String>,
    source_invoice_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
    lines: Option<Vec<PostV1InventoryLandedCostsCreateResponseLinesItem>>,
}

impl PostV1InventoryLandedCostsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn method(mut self, value: PostV1InventoryLandedCostsCreateResponseMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn goods_receipt_id(mut self, value: impl Into<String>) -> Self {
        self.goods_receipt_id = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1InventoryLandedCostsCreateResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLandedCostsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryLandedCostsCreateResponseBuilder::id)
    /// - [`date`](PostV1InventoryLandedCostsCreateResponseBuilder::date)
    /// - [`amount`](PostV1InventoryLandedCostsCreateResponseBuilder::amount)
    /// - [`method`](PostV1InventoryLandedCostsCreateResponseBuilder::method)
    /// - [`created_at`](PostV1InventoryLandedCostsCreateResponseBuilder::created_at)
    /// - [`lines`](PostV1InventoryLandedCostsCreateResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1InventoryLandedCostsCreateResponse, BuildError> {
        Ok(PostV1InventoryLandedCostsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            goods_receipt_id: self.goods_receipt_id,
            source_invoice_id: self.source_invoice_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
