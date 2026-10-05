pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesOverridesDeleteReferenceRequest {
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub date: NaiveDate,
}

impl ExchangeRatesOverridesDeleteReferenceRequest {
    pub fn builder() -> ExchangeRatesOverridesDeleteReferenceRequestBuilder {
        <ExchangeRatesOverridesDeleteReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesOverridesDeleteReferenceRequestBuilder {
    currency: Option<String>,
    date: Option<NaiveDate>,
}

impl ExchangeRatesOverridesDeleteReferenceRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExchangeRatesOverridesDeleteReferenceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](ExchangeRatesOverridesDeleteReferenceRequestBuilder::currency)
    /// - [`date`](ExchangeRatesOverridesDeleteReferenceRequestBuilder::date)
    pub fn build(self) -> Result<ExchangeRatesOverridesDeleteReferenceRequest, BuildError> {
        Ok(ExchangeRatesOverridesDeleteReferenceRequest {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
        })
    }
}
