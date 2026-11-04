pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundsCreatePosResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "shiftId")]
    #[serde(default)]
    pub shift_id: String,
    #[serde(rename = "receiptId")]
    #[serde(default)]
    pub receipt_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(default)]
    pub vat_total: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "cashAmount")]
    #[serde(default)]
    pub cash_amount: String,
    #[serde(rename = "cogsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cogs_total: Option<String>,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub lines: Vec<RefundsCreatePosResponseLinesItem>,
}

impl RefundsCreatePosResponse {
    pub fn builder() -> RefundsCreatePosResponseBuilder {
        <RefundsCreatePosResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundsCreatePosResponseBuilder {
    id: Option<String>,
    shift_id: Option<String>,
    receipt_id: Option<String>,
    date: Option<NaiveDate>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    cash_amount: Option<String>,
    cogs_total: Option<String>,
    journal_transaction_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    lines: Option<Vec<RefundsCreatePosResponseLinesItem>>,
}

impl RefundsCreatePosResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn shift_id(mut self, value: impl Into<String>) -> Self {
        self.shift_id = Some(value.into());
        self
    }

    pub fn receipt_id(mut self, value: impl Into<String>) -> Self {
        self.receipt_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn vat_total(mut self, value: impl Into<String>) -> Self {
        self.vat_total = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn cash_amount(mut self, value: impl Into<String>) -> Self {
        self.cash_amount = Some(value.into());
        self
    }

    pub fn cogs_total(mut self, value: impl Into<String>) -> Self {
        self.cogs_total = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<RefundsCreatePosResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundsCreatePosResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundsCreatePosResponseBuilder::id)
    /// - [`shift_id`](RefundsCreatePosResponseBuilder::shift_id)
    /// - [`receipt_id`](RefundsCreatePosResponseBuilder::receipt_id)
    /// - [`date`](RefundsCreatePosResponseBuilder::date)
    /// - [`net_total`](RefundsCreatePosResponseBuilder::net_total)
    /// - [`vat_total`](RefundsCreatePosResponseBuilder::vat_total)
    /// - [`gross_total`](RefundsCreatePosResponseBuilder::gross_total)
    /// - [`cash_amount`](RefundsCreatePosResponseBuilder::cash_amount)
    /// - [`created_at`](RefundsCreatePosResponseBuilder::created_at)
    /// - [`lines`](RefundsCreatePosResponseBuilder::lines)
    pub fn build(self) -> Result<RefundsCreatePosResponse, BuildError> {
        Ok(RefundsCreatePosResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            shift_id: self
                .shift_id
                .ok_or_else(|| BuildError::missing_field("shift_id"))?,
            receipt_id: self
                .receipt_id
                .ok_or_else(|| BuildError::missing_field("receipt_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            vat_total: self
                .vat_total
                .ok_or_else(|| BuildError::missing_field("vat_total"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            cash_amount: self
                .cash_amount
                .ok_or_else(|| BuildError::missing_field("cash_amount"))?,
            cogs_total: self.cogs_total,
            journal_transaction_id: self.journal_transaction_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
