pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionsListBillingResponseRowsItem {
    #[serde(default)]
    pub id: String,
    pub r#type: TransactionsListBillingResponseRowsItemType,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(rename = "balanceAfterCents")]
    #[serde(default)]
    pub balance_after_cents: i64,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "usageDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<NaiveDate>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl TransactionsListBillingResponseRowsItem {
    pub fn builder() -> TransactionsListBillingResponseRowsItemBuilder {
        <TransactionsListBillingResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsListBillingResponseRowsItemBuilder {
    id: Option<String>,
    r#type: Option<TransactionsListBillingResponseRowsItemType>,
    amount_cents: Option<i64>,
    balance_after_cents: Option<i64>,
    description: Option<String>,
    reference: Option<String>,
    usage_date: Option<NaiveDate>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl TransactionsListBillingResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TransactionsListBillingResponseRowsItemType) -> Self {
        self.r#type = Some(value);
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

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn usage_date(mut self, value: NaiveDate) -> Self {
        self.usage_date = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsListBillingResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TransactionsListBillingResponseRowsItemBuilder::id)
    /// - [`r#type`](TransactionsListBillingResponseRowsItemBuilder::r#type)
    /// - [`amount_cents`](TransactionsListBillingResponseRowsItemBuilder::amount_cents)
    /// - [`balance_after_cents`](TransactionsListBillingResponseRowsItemBuilder::balance_after_cents)
    /// - [`description`](TransactionsListBillingResponseRowsItemBuilder::description)
    /// - [`created_at`](TransactionsListBillingResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<TransactionsListBillingResponseRowsItem, BuildError> {
        Ok(TransactionsListBillingResponseRowsItem {
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
            reference: self.reference,
            usage_date: self.usage_date,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
