pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountSetPlanBillingResponseAutoTopUp {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(default)]
    pub pending: bool,
}

impl AccountSetPlanBillingResponseAutoTopUp {
    pub fn builder() -> AccountSetPlanBillingResponseAutoTopUpBuilder {
        <AccountSetPlanBillingResponseAutoTopUpBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSetPlanBillingResponseAutoTopUpBuilder {
    enabled: Option<bool>,
    amount_cents: Option<i64>,
    pending: Option<bool>,
}

impl AccountSetPlanBillingResponseAutoTopUpBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn pending(mut self, value: bool) -> Self {
        self.pending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountSetPlanBillingResponseAutoTopUp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](AccountSetPlanBillingResponseAutoTopUpBuilder::enabled)
    /// - [`amount_cents`](AccountSetPlanBillingResponseAutoTopUpBuilder::amount_cents)
    /// - [`pending`](AccountSetPlanBillingResponseAutoTopUpBuilder::pending)
    pub fn build(self) -> Result<AccountSetPlanBillingResponseAutoTopUp, BuildError> {
        Ok(AccountSetPlanBillingResponseAutoTopUp {
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            pending: self
                .pending
                .ok_or_else(|| BuildError::missing_field("pending"))?,
        })
    }
}
