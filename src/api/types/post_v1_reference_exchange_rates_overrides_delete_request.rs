pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceExchangeRatesOverridesDeleteRequest {
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub date: String,
}

impl PostV1ReferenceExchangeRatesOverridesDeleteRequest {
    pub fn builder() -> PostV1ReferenceExchangeRatesOverridesDeleteRequestBuilder {
        <PostV1ReferenceExchangeRatesOverridesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesOverridesDeleteRequestBuilder {
    currency: Option<String>,
    date: Option<String>,
}

impl PostV1ReferenceExchangeRatesOverridesDeleteRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesOverridesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](PostV1ReferenceExchangeRatesOverridesDeleteRequestBuilder::currency)
    /// - [`date`](PostV1ReferenceExchangeRatesOverridesDeleteRequestBuilder::date)
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesOverridesDeleteRequest, BuildError> {
        Ok(PostV1ReferenceExchangeRatesOverridesDeleteRequest {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
        })
    }
}
