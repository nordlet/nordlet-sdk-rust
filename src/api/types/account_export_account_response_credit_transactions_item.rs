pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountResponseCreditTransactionsItem {
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ExportAccountResponseCreditTransactionsItem {
    pub fn builder() -> ExportAccountResponseCreditTransactionsItemBuilder {
        <ExportAccountResponseCreditTransactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseCreditTransactionsItemBuilder {
    id: Option<String>,
    r#type: Option<String>,
    amount_cents: Option<i64>,
    balance_after_cents: Option<i64>,
    description: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ExportAccountResponseCreditTransactionsItemBuilder {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportAccountResponseCreditTransactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExportAccountResponseCreditTransactionsItemBuilder::id)
    /// - [`r#type`](ExportAccountResponseCreditTransactionsItemBuilder::r#type)
    /// - [`amount_cents`](ExportAccountResponseCreditTransactionsItemBuilder::amount_cents)
    /// - [`balance_after_cents`](ExportAccountResponseCreditTransactionsItemBuilder::balance_after_cents)
    /// - [`description`](ExportAccountResponseCreditTransactionsItemBuilder::description)
    /// - [`created_at`](ExportAccountResponseCreditTransactionsItemBuilder::created_at)
    pub fn build(self) -> Result<ExportAccountResponseCreditTransactionsItem, BuildError> {
        Ok(ExportAccountResponseCreditTransactionsItem {
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
