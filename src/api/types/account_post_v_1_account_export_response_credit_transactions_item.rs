pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountExportResponseCreditTransactionsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(rename = "balanceAfterCents")]
    #[serde(default)]
    pub balance_after_cents: i64,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1AccountExportResponseCreditTransactionsItem {
    pub fn builder() -> PostV1AccountExportResponseCreditTransactionsItemBuilder {
        <PostV1AccountExportResponseCreditTransactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountExportResponseCreditTransactionsItemBuilder {
    id: Option<String>,
    r#type: Option<String>,
    amount_cents: Option<i64>,
    balance_after_cents: Option<i64>,
    description: Option<String>,
    created_at: Option<String>,
}

impl PostV1AccountExportResponseCreditTransactionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn balance_after_cents(mut self, value: i64) -> Self {
        self.balance_after_cents = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountExportResponseCreditTransactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountExportResponseCreditTransactionsItemBuilder::id)
    /// - [`r#type`](PostV1AccountExportResponseCreditTransactionsItemBuilder::r#type)
    /// - [`amount_cents`](PostV1AccountExportResponseCreditTransactionsItemBuilder::amount_cents)
    /// - [`balance_after_cents`](PostV1AccountExportResponseCreditTransactionsItemBuilder::balance_after_cents)
    /// - [`description`](PostV1AccountExportResponseCreditTransactionsItemBuilder::description)
    /// - [`created_at`](PostV1AccountExportResponseCreditTransactionsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1AccountExportResponseCreditTransactionsItem, BuildError> {
        Ok(PostV1AccountExportResponseCreditTransactionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            balance_after_cents: self
                .balance_after_cents
                .ok_or_else(|| BuildError::missing_field("balance_after_cents"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
