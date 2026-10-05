pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LandedCostsCreateInventoryResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub amount: String,
    pub method: LandedCostsCreateInventoryResponseMethod,
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub lines: Vec<LandedCostsCreateInventoryResponseLinesItem>,
}

impl LandedCostsCreateInventoryResponse {
    pub fn builder() -> LandedCostsCreateInventoryResponseBuilder {
        <LandedCostsCreateInventoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandedCostsCreateInventoryResponseBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    amount: Option<String>,
    method: Option<LandedCostsCreateInventoryResponseMethod>,
    goods_receipt_id: Option<String>,
    source_invoice_id: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    lines: Option<Vec<LandedCostsCreateInventoryResponseLinesItem>>,
}

impl LandedCostsCreateInventoryResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn method(mut self, value: LandedCostsCreateInventoryResponseMethod) -> Self {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<LandedCostsCreateInventoryResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandedCostsCreateInventoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LandedCostsCreateInventoryResponseBuilder::id)
    /// - [`date`](LandedCostsCreateInventoryResponseBuilder::date)
    /// - [`amount`](LandedCostsCreateInventoryResponseBuilder::amount)
    /// - [`method`](LandedCostsCreateInventoryResponseBuilder::method)
    /// - [`created_at`](LandedCostsCreateInventoryResponseBuilder::created_at)
    /// - [`lines`](LandedCostsCreateInventoryResponseBuilder::lines)
    pub fn build(self) -> Result<LandedCostsCreateInventoryResponse, BuildError> {
        Ok(LandedCostsCreateInventoryResponse {
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
