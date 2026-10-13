pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TopupCreateBillingRequest {
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<TopupCreateBillingRequestLocale>,
    #[serde(rename = "returnTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_to: Option<TopupCreateBillingRequestReturnTo>,
}

impl TopupCreateBillingRequest {
    pub fn builder() -> TopupCreateBillingRequestBuilder {
        <TopupCreateBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopupCreateBillingRequestBuilder {
    amount_cents: Option<i64>,
    locale: Option<TopupCreateBillingRequestLocale>,
    return_to: Option<TopupCreateBillingRequestReturnTo>,
}

impl TopupCreateBillingRequestBuilder {
    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn locale(mut self, value: TopupCreateBillingRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn return_to(mut self, value: TopupCreateBillingRequestReturnTo) -> Self {
        self.return_to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TopupCreateBillingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_cents`](TopupCreateBillingRequestBuilder::amount_cents)
    pub fn build(self) -> Result<TopupCreateBillingRequest, BuildError> {
        Ok(TopupCreateBillingRequest {
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            locale: self.locale,
            return_to: self.return_to,
        })
    }
}
