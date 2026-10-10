pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LandedCostsGetInventoryResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub amount: String,
    pub method: LandedCostsGetInventoryResponseMethod,
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
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(default)]
    pub lines: Vec<LandedCostsGetInventoryResponseLinesItem>,
}

impl LandedCostsGetInventoryResponse {
    pub fn builder() -> LandedCostsGetInventoryResponseBuilder {
        <LandedCostsGetInventoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandedCostsGetInventoryResponseBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    amount: Option<String>,
    method: Option<LandedCostsGetInventoryResponseMethod>,
    goods_receipt_id: Option<String>,
    source_invoice_id: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    journal_transaction_id: Option<String>,
    lines: Option<Vec<LandedCostsGetInventoryResponseLinesItem>>,
}

impl LandedCostsGetInventoryResponseBuilder {
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

    pub fn method(mut self, value: LandedCostsGetInventoryResponseMethod) -> Self {
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

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<LandedCostsGetInventoryResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandedCostsGetInventoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LandedCostsGetInventoryResponseBuilder::id)
    /// - [`date`](LandedCostsGetInventoryResponseBuilder::date)
    /// - [`amount`](LandedCostsGetInventoryResponseBuilder::amount)
    /// - [`method`](LandedCostsGetInventoryResponseBuilder::method)
    /// - [`created_at`](LandedCostsGetInventoryResponseBuilder::created_at)
    /// - [`lines`](LandedCostsGetInventoryResponseBuilder::lines)
    pub fn build(self) -> Result<LandedCostsGetInventoryResponse, BuildError> {
        Ok(LandedCostsGetInventoryResponse {
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
            journal_transaction_id: self.journal_transaction_id,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
