pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountSetAutoTopUpBillingRequest {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
}

impl AccountSetAutoTopUpBillingRequest {
    pub fn builder() -> AccountSetAutoTopUpBillingRequestBuilder {
        <AccountSetAutoTopUpBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSetAutoTopUpBillingRequestBuilder {
    enabled: Option<bool>,
    amount_cents: Option<i64>,
}

impl AccountSetAutoTopUpBillingRequestBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountSetAutoTopUpBillingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](AccountSetAutoTopUpBillingRequestBuilder::enabled)
    /// - [`amount_cents`](AccountSetAutoTopUpBillingRequestBuilder::amount_cents)
    pub fn build(self) -> Result<AccountSetAutoTopUpBillingRequest, BuildError> {
        Ok(AccountSetAutoTopUpBillingRequest {
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
        })
    }
}
