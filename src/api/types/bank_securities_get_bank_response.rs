pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SecuritiesGetBankResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(default)]
    pub broker: String,
    #[serde(rename = "brokerAccount")]
    #[serde(default)]
    pub broker_account: String,
    #[serde(default)]
    pub period: String,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "operationCount")]
    #[serde(default)]
    pub operation_count: i64,
    #[serde(rename = "duplicateCount")]
    #[serde(default)]
    pub duplicate_count: i64,
    #[serde(rename = "postedCount")]
    #[serde(default)]
    pub posted_count: i64,
    #[serde(rename = "notPostedCount")]
    #[serde(default)]
    pub not_posted_count: i64,
    #[serde(rename = "realizedTotal")]
    #[serde(default)]
    pub realized_total: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub operations: Vec<SecuritiesGetBankResponseOperationsItem>,
}

impl SecuritiesGetBankResponse {
    pub fn builder() -> SecuritiesGetBankResponseBuilder {
        <SecuritiesGetBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecuritiesGetBankResponseBuilder {
    id: Option<String>,
    bank_account_id: Option<String>,
    broker: Option<String>,
    broker_account: Option<String>,
    period: Option<String>,
    file_name: Option<String>,
    operation_count: Option<i64>,
    duplicate_count: Option<i64>,
    posted_count: Option<i64>,
    not_posted_count: Option<i64>,
    realized_total: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    operations: Option<Vec<SecuritiesGetBankResponseOperationsItem>>,
}

impl SecuritiesGetBankResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn broker(mut self, value: impl Into<String>) -> Self {
        self.broker = Some(value.into());
        self
    }

    pub fn broker_account(mut self, value: impl Into<String>) -> Self {
        self.broker_account = Some(value.into());
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn operation_count(mut self, value: i64) -> Self {
        self.operation_count = Some(value);
        self
    }

    pub fn duplicate_count(mut self, value: i64) -> Self {
        self.duplicate_count = Some(value);
        self
    }

    pub fn posted_count(mut self, value: i64) -> Self {
        self.posted_count = Some(value);
        self
    }

    pub fn not_posted_count(mut self, value: i64) -> Self {
        self.not_posted_count = Some(value);
        self
    }

    pub fn realized_total(mut self, value: impl Into<String>) -> Self {
        self.realized_total = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn operations(mut self, value: Vec<SecuritiesGetBankResponseOperationsItem>) -> Self {
        self.operations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SecuritiesGetBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SecuritiesGetBankResponseBuilder::id)
    /// - [`bank_account_id`](SecuritiesGetBankResponseBuilder::bank_account_id)
    /// - [`broker`](SecuritiesGetBankResponseBuilder::broker)
    /// - [`broker_account`](SecuritiesGetBankResponseBuilder::broker_account)
    /// - [`period`](SecuritiesGetBankResponseBuilder::period)
    /// - [`operation_count`](SecuritiesGetBankResponseBuilder::operation_count)
    /// - [`duplicate_count`](SecuritiesGetBankResponseBuilder::duplicate_count)
    /// - [`posted_count`](SecuritiesGetBankResponseBuilder::posted_count)
    /// - [`not_posted_count`](SecuritiesGetBankResponseBuilder::not_posted_count)
    /// - [`realized_total`](SecuritiesGetBankResponseBuilder::realized_total)
    /// - [`created_at`](SecuritiesGetBankResponseBuilder::created_at)
    /// - [`operations`](SecuritiesGetBankResponseBuilder::operations)
    pub fn build(self) -> Result<SecuritiesGetBankResponse, BuildError> {
        Ok(SecuritiesGetBankResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            broker: self
                .broker
                .ok_or_else(|| BuildError::missing_field("broker"))?,
            broker_account: self
                .broker_account
                .ok_or_else(|| BuildError::missing_field("broker_account"))?,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
            file_name: self.file_name,
            operation_count: self
                .operation_count
                .ok_or_else(|| BuildError::missing_field("operation_count"))?,
            duplicate_count: self
                .duplicate_count
                .ok_or_else(|| BuildError::missing_field("duplicate_count"))?,
            posted_count: self
                .posted_count
                .ok_or_else(|| BuildError::missing_field("posted_count"))?,
            not_posted_count: self
                .not_posted_count
                .ok_or_else(|| BuildError::missing_field("not_posted_count"))?,
            realized_total: self
                .realized_total
                .ok_or_else(|| BuildError::missing_field("realized_total"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            operations: self
                .operations
                .ok_or_else(|| BuildError::missing_field("operations"))?,
        })
    }
}
