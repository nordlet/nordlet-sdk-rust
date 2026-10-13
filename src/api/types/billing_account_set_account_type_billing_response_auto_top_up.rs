pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountSetAccountTypeBillingResponseAutoTopUp {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(default)]
    pub pending: bool,
}

impl AccountSetAccountTypeBillingResponseAutoTopUp {
    pub fn builder() -> AccountSetAccountTypeBillingResponseAutoTopUpBuilder {
        <AccountSetAccountTypeBillingResponseAutoTopUpBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSetAccountTypeBillingResponseAutoTopUpBuilder {
    enabled: Option<bool>,
    amount_cents: Option<i64>,
    pending: Option<bool>,
}

impl AccountSetAccountTypeBillingResponseAutoTopUpBuilder {
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

    /// Consumes the builder and constructs a [`AccountSetAccountTypeBillingResponseAutoTopUp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](AccountSetAccountTypeBillingResponseAutoTopUpBuilder::enabled)
    /// - [`amount_cents`](AccountSetAccountTypeBillingResponseAutoTopUpBuilder::amount_cents)
    /// - [`pending`](AccountSetAccountTypeBillingResponseAutoTopUpBuilder::pending)
    pub fn build(self) -> Result<AccountSetAccountTypeBillingResponseAutoTopUp, BuildError> {
        Ok(AccountSetAccountTypeBillingResponseAutoTopUp {
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
