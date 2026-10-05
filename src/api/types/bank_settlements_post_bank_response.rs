pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SettlementsPostBankResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(default)]
    pub provider: String,
    #[serde(rename = "payoutId")]
    #[serde(default)]
    pub payout_id: String,
    #[serde(rename = "payoutDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_date: Option<NaiveDate>,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "feeTotal")]
    #[serde(default)]
    pub fee_total: String,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "fxRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx_rate: Option<String>,
    pub status: SettlementsPostBankResponseStatus,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(rename = "bankTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transaction_id: Option<String>,
    #[serde(rename = "lineCount")]
    #[serde(default)]
    pub line_count: i64,
    #[serde(rename = "matchedCount")]
    #[serde(default)]
    pub matched_count: i64,
    #[serde(rename = "unmatchedCount")]
    #[serde(default)]
    pub unmatched_count: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub summary: SettlementsPostBankResponseSummary,
}

impl SettlementsPostBankResponse {
    pub fn builder() -> SettlementsPostBankResponseBuilder {
        <SettlementsPostBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsPostBankResponseBuilder {
    id: Option<String>,
    bank_account_id: Option<String>,
    provider: Option<String>,
    payout_id: Option<String>,
    payout_date: Option<NaiveDate>,
    currency: Option<String>,
    gross_total: Option<String>,
    fee_total: Option<String>,
    net_total: Option<String>,
    fx_rate: Option<String>,
    status: Option<SettlementsPostBankResponseStatus>,
    journal_transaction_id: Option<String>,
    bank_transaction_id: Option<String>,
    line_count: Option<i64>,
    matched_count: Option<i64>,
    unmatched_count: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    warnings: Option<Vec<String>>,
    summary: Option<SettlementsPostBankResponseSummary>,
}

impl SettlementsPostBankResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn payout_id(mut self, value: impl Into<String>) -> Self {
        self.payout_id = Some(value.into());
        self
    }

    pub fn payout_date(mut self, value: NaiveDate) -> Self {
        self.payout_date = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn fee_total(mut self, value: impl Into<String>) -> Self {
        self.fee_total = Some(value.into());
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn fx_rate(mut self, value: impl Into<String>) -> Self {
        self.fx_rate = Some(value.into());
        self
    }

    pub fn status(mut self, value: SettlementsPostBankResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn bank_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.bank_transaction_id = Some(value.into());
        self
    }

    pub fn line_count(mut self, value: i64) -> Self {
        self.line_count = Some(value);
        self
    }

    pub fn matched_count(mut self, value: i64) -> Self {
        self.matched_count = Some(value);
        self
    }

    pub fn unmatched_count(mut self, value: i64) -> Self {
        self.unmatched_count = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn summary(mut self, value: SettlementsPostBankResponseSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettlementsPostBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SettlementsPostBankResponseBuilder::id)
    /// - [`bank_account_id`](SettlementsPostBankResponseBuilder::bank_account_id)
    /// - [`provider`](SettlementsPostBankResponseBuilder::provider)
    /// - [`payout_id`](SettlementsPostBankResponseBuilder::payout_id)
    /// - [`currency`](SettlementsPostBankResponseBuilder::currency)
    /// - [`gross_total`](SettlementsPostBankResponseBuilder::gross_total)
    /// - [`fee_total`](SettlementsPostBankResponseBuilder::fee_total)
    /// - [`net_total`](SettlementsPostBankResponseBuilder::net_total)
    /// - [`status`](SettlementsPostBankResponseBuilder::status)
    /// - [`line_count`](SettlementsPostBankResponseBuilder::line_count)
    /// - [`matched_count`](SettlementsPostBankResponseBuilder::matched_count)
    /// - [`unmatched_count`](SettlementsPostBankResponseBuilder::unmatched_count)
    /// - [`created_at`](SettlementsPostBankResponseBuilder::created_at)
    /// - [`updated_at`](SettlementsPostBankResponseBuilder::updated_at)
    /// - [`warnings`](SettlementsPostBankResponseBuilder::warnings)
    /// - [`summary`](SettlementsPostBankResponseBuilder::summary)
    pub fn build(self) -> Result<SettlementsPostBankResponse, BuildError> {
        Ok(SettlementsPostBankResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            payout_id: self
                .payout_id
                .ok_or_else(|| BuildError::missing_field("payout_id"))?,
            payout_date: self.payout_date,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            fee_total: self
                .fee_total
                .ok_or_else(|| BuildError::missing_field("fee_total"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            fx_rate: self.fx_rate,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            journal_transaction_id: self.journal_transaction_id,
            bank_transaction_id: self.bank_transaction_id,
            line_count: self
                .line_count
                .ok_or_else(|| BuildError::missing_field("line_count"))?,
            matched_count: self
                .matched_count
                .ok_or_else(|| BuildError::missing_field("matched_count"))?,
            unmatched_count: self
                .unmatched_count
                .ok_or_else(|| BuildError::missing_field("unmatched_count"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
