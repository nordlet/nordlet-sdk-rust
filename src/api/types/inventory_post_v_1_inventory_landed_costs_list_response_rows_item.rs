pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLandedCostsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub amount: String,
    pub method: PostV1InventoryLandedCostsListResponseRowsItemMethod,
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
}

impl PostV1InventoryLandedCostsListResponseRowsItem {
    pub fn builder() -> PostV1InventoryLandedCostsListResponseRowsItemBuilder {
        <PostV1InventoryLandedCostsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLandedCostsListResponseRowsItemBuilder {
    id: Option<String>,
    date: Option<String>,
    amount: Option<String>,
    method: Option<PostV1InventoryLandedCostsListResponseRowsItemMethod>,
    goods_receipt_id: Option<String>,
    source_invoice_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1InventoryLandedCostsListResponseRowsItemBuilder {
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

    pub fn method(mut self, value: PostV1InventoryLandedCostsListResponseRowsItemMethod) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1InventoryLandedCostsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryLandedCostsListResponseRowsItemBuilder::id)
    /// - [`date`](PostV1InventoryLandedCostsListResponseRowsItemBuilder::date)
    /// - [`amount`](PostV1InventoryLandedCostsListResponseRowsItemBuilder::amount)
    /// - [`method`](PostV1InventoryLandedCostsListResponseRowsItemBuilder::method)
    /// - [`created_at`](PostV1InventoryLandedCostsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1InventoryLandedCostsListResponseRowsItem, BuildError> {
        Ok(PostV1InventoryLandedCostsListResponseRowsItem {
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
        })
    }
}
