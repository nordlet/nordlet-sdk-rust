pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLandedCostsGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub amount: String,
    pub method: PostV1InventoryLandedCostsGetResponseMethod,
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
    pub lines: Vec<PostV1InventoryLandedCostsGetResponseLinesItem>,
}

impl PostV1InventoryLandedCostsGetResponse {
    pub fn builder() -> PostV1InventoryLandedCostsGetResponseBuilder {
        <PostV1InventoryLandedCostsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLandedCostsGetResponseBuilder {
    id: Option<String>,
    date: Option<String>,
    amount: Option<String>,
    method: Option<PostV1InventoryLandedCostsGetResponseMethod>,
    goods_receipt_id: Option<String>,
    source_invoice_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
    lines: Option<Vec<PostV1InventoryLandedCostsGetResponseLinesItem>>,
}

impl PostV1InventoryLandedCostsGetResponseBuilder {
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

    pub fn method(mut self, value: PostV1InventoryLandedCostsGetResponseMethod) -> Self {
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

    pub fn lines(mut self, value: Vec<PostV1InventoryLandedCostsGetResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLandedCostsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryLandedCostsGetResponseBuilder::id)
    /// - [`date`](PostV1InventoryLandedCostsGetResponseBuilder::date)
    /// - [`amount`](PostV1InventoryLandedCostsGetResponseBuilder::amount)
    /// - [`method`](PostV1InventoryLandedCostsGetResponseBuilder::method)
    /// - [`created_at`](PostV1InventoryLandedCostsGetResponseBuilder::created_at)
    /// - [`lines`](PostV1InventoryLandedCostsGetResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1InventoryLandedCostsGetResponse, BuildError> {
        Ok(PostV1InventoryLandedCostsGetResponse {
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
