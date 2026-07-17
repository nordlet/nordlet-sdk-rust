pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsListResponseRowsItem {
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
    pub payout_date: Option<String>,
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
    pub status: PostV1BankSettlementsListResponseRowsItemStatus,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
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
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1BankSettlementsListResponseRowsItem {
    pub fn builder() -> PostV1BankSettlementsListResponseRowsItemBuilder {
        <PostV1BankSettlementsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsListResponseRowsItemBuilder {
    id: Option<String>,
    bank_account_id: Option<String>,
    provider: Option<String>,
    payout_id: Option<String>,
    payout_date: Option<String>,
    currency: Option<String>,
    gross_total: Option<String>,
    fee_total: Option<String>,
    net_total: Option<String>,
    status: Option<PostV1BankSettlementsListResponseRowsItemStatus>,
    journal_transaction_id: Option<String>,
    line_count: Option<i64>,
    matched_count: Option<i64>,
    unmatched_count: Option<i64>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1BankSettlementsListResponseRowsItemBuilder {
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

    pub fn payout_date(mut self, value: impl Into<String>) -> Self {
        self.payout_date = Some(value.into());
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

    pub fn status(mut self, value: PostV1BankSettlementsListResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankSettlementsListResponseRowsItemBuilder::id)
    /// - [`bank_account_id`](PostV1BankSettlementsListResponseRowsItemBuilder::bank_account_id)
    /// - [`provider`](PostV1BankSettlementsListResponseRowsItemBuilder::provider)
    /// - [`payout_id`](PostV1BankSettlementsListResponseRowsItemBuilder::payout_id)
    /// - [`currency`](PostV1BankSettlementsListResponseRowsItemBuilder::currency)
    /// - [`gross_total`](PostV1BankSettlementsListResponseRowsItemBuilder::gross_total)
    /// - [`fee_total`](PostV1BankSettlementsListResponseRowsItemBuilder::fee_total)
    /// - [`net_total`](PostV1BankSettlementsListResponseRowsItemBuilder::net_total)
    /// - [`status`](PostV1BankSettlementsListResponseRowsItemBuilder::status)
    /// - [`line_count`](PostV1BankSettlementsListResponseRowsItemBuilder::line_count)
    /// - [`matched_count`](PostV1BankSettlementsListResponseRowsItemBuilder::matched_count)
    /// - [`unmatched_count`](PostV1BankSettlementsListResponseRowsItemBuilder::unmatched_count)
    /// - [`created_at`](PostV1BankSettlementsListResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](PostV1BankSettlementsListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1BankSettlementsListResponseRowsItem, BuildError> {
        Ok(PostV1BankSettlementsListResponseRowsItem {
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
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            journal_transaction_id: self.journal_transaction_id,
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
        })
    }
}
