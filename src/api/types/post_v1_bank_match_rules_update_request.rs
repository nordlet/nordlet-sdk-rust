pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMatchRulesUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "payoutIdPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_id_prefix: Option<String>,
    #[serde(rename = "bankAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_id: Option<String>,
    #[serde(rename = "dateWindowDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_window_days: Option<i64>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl PostV1BankMatchRulesUpdateRequest {
    pub fn builder() -> PostV1BankMatchRulesUpdateRequestBuilder {
        <PostV1BankMatchRulesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMatchRulesUpdateRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    provider: Option<String>,
    pattern: Option<String>,
    payout_id_prefix: Option<String>,
    bank_account_id: Option<String>,
    date_window_days: Option<i64>,
    is_active: Option<bool>,
}

impl PostV1BankMatchRulesUpdateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1BankMatchRulesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankMatchRulesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankMatchRulesUpdateRequest, BuildError> {
        Ok(PostV1BankMatchRulesUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            provider: self.provider,
            pattern: self.pattern,
            payout_id_prefix: self.payout_id_prefix,
            bank_account_id: self.bank_account_id,
            date_window_days: self.date_window_days,
            is_active: self.is_active,
        })
    }
}
