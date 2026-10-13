pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TopupConfirmBillingResponseAutoTopUp {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(default)]
    pub pending: bool,
}

impl TopupConfirmBillingResponseAutoTopUp {
    pub fn builder() -> TopupConfirmBillingResponseAutoTopUpBuilder {
        <TopupConfirmBillingResponseAutoTopUpBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopupConfirmBillingResponseAutoTopUpBuilder {
    enabled: Option<bool>,
    amount_cents: Option<i64>,
    pending: Option<bool>,
}

impl TopupConfirmBillingResponseAutoTopUpBuilder {
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

    /// Consumes the builder and constructs a [`TopupConfirmBillingResponseAutoTopUp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](TopupConfirmBillingResponseAutoTopUpBuilder::enabled)
    /// - [`amount_cents`](TopupConfirmBillingResponseAutoTopUpBuilder::amount_cents)
    /// - [`pending`](TopupConfirmBillingResponseAutoTopUpBuilder::pending)
    pub fn build(self) -> Result<TopupConfirmBillingResponseAutoTopUp, BuildError> {
        Ok(TopupConfirmBillingResponseAutoTopUp {
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
