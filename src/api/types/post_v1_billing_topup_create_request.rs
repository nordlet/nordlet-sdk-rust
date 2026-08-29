pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BillingTopupCreateRequest {
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1BillingTopupCreateRequestLocale>,
}

impl PostV1BillingTopupCreateRequest {
    pub fn builder() -> PostV1BillingTopupCreateRequestBuilder {
        <PostV1BillingTopupCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingTopupCreateRequestBuilder {
    amount_cents: Option<i64>,
    locale: Option<PostV1BillingTopupCreateRequestLocale>,
}

impl PostV1BillingTopupCreateRequestBuilder {
    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn locale(mut self, value: PostV1BillingTopupCreateRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingTopupCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_cents`](PostV1BillingTopupCreateRequestBuilder::amount_cents)
    pub fn build(self) -> Result<PostV1BillingTopupCreateRequest, BuildError> {
        Ok(PostV1BillingTopupCreateRequest {
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            locale: self.locale,
        })
    }
}
