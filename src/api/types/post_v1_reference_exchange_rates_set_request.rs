pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceExchangeRatesSetRequest {
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub rate: String,
}

impl PostV1ReferenceExchangeRatesSetRequest {
    pub fn builder() -> PostV1ReferenceExchangeRatesSetRequestBuilder {
        <PostV1ReferenceExchangeRatesSetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesSetRequestBuilder {
    currency: Option<String>,
    date: Option<String>,
    rate: Option<String>,
}

impl PostV1ReferenceExchangeRatesSetRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesSetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](PostV1ReferenceExchangeRatesSetRequestBuilder::currency)
    /// - [`date`](PostV1ReferenceExchangeRatesSetRequestBuilder::date)
    /// - [`rate`](PostV1ReferenceExchangeRatesSetRequestBuilder::rate)
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesSetRequest, BuildError> {
        Ok(PostV1ReferenceExchangeRatesSetRequest {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
        })
    }
}
