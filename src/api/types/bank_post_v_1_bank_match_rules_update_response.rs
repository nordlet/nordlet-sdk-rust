pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMatchRulesUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub pattern: String,
    #[serde(rename = "payoutIdPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_id_prefix: Option<String>,
    #[serde(rename = "bankAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_id: Option<String>,
    #[serde(rename = "dateWindowDays")]
    #[serde(default)]
    pub date_window_days: i64,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1BankMatchRulesUpdateResponse {
    pub fn builder() -> PostV1BankMatchRulesUpdateResponseBuilder {
        <PostV1BankMatchRulesUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMatchRulesUpdateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    provider: Option<String>,
    pattern: Option<String>,
    payout_id_prefix: Option<String>,
    bank_account_id: Option<String>,
    date_window_days: Option<i64>,
    is_active: Option<bool>,
    created_at: Option<String>,
}

impl PostV1BankMatchRulesUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }

    pub fn payout_id_prefix(mut self, value: impl Into<String>) -> Self {
        self.payout_id_prefix = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn date_window_days(mut self, value: i64) -> Self {
        self.date_window_days = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankMatchRulesUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankMatchRulesUpdateResponseBuilder::id)
    /// - [`name`](PostV1BankMatchRulesUpdateResponseBuilder::name)
    /// - [`provider`](PostV1BankMatchRulesUpdateResponseBuilder::provider)
    /// - [`pattern`](PostV1BankMatchRulesUpdateResponseBuilder::pattern)
    /// - [`date_window_days`](PostV1BankMatchRulesUpdateResponseBuilder::date_window_days)
    /// - [`is_active`](PostV1BankMatchRulesUpdateResponseBuilder::is_active)
    /// - [`created_at`](PostV1BankMatchRulesUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1BankMatchRulesUpdateResponse, BuildError> {
        Ok(PostV1BankMatchRulesUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            pattern: self
                .pattern
                .ok_or_else(|| BuildError::missing_field("pattern"))?,
            payout_id_prefix: self.payout_id_prefix,
            bank_account_id: self.bank_account_id,
            date_window_days: self
                .date_window_days
                .ok_or_else(|| BuildError::missing_field("date_window_days"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
