pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountSetOverdraftCapBillingRequest {
    #[serde(rename = "capCents")]
    #[serde(default)]
    pub cap_cents: i64,
}

impl AccountSetOverdraftCapBillingRequest {
    pub fn builder() -> AccountSetOverdraftCapBillingRequestBuilder {
        <AccountSetOverdraftCapBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSetOverdraftCapBillingRequestBuilder {
    cap_cents: Option<i64>,
}

impl AccountSetOverdraftCapBillingRequestBuilder {
    pub fn cap_cents(mut self, value: i64) -> Self {
        self.cap_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountSetOverdraftCapBillingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cap_cents`](AccountSetOverdraftCapBillingRequestBuilder::cap_cents)
    pub fn build(self) -> Result<AccountSetOverdraftCapBillingRequest, BuildError> {
        Ok(AccountSetOverdraftCapBillingRequest {
            cap_cents: self
                .cap_cents
                .ok_or_else(|| BuildError::missing_field("cap_cents"))?,
        })
    }
}
