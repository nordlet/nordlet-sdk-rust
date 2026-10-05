pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesSetReferenceRequest {
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub rate: String,
}

impl ExchangeRatesSetReferenceRequest {
    pub fn builder() -> ExchangeRatesSetReferenceRequestBuilder {
        <ExchangeRatesSetReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesSetReferenceRequestBuilder {
    currency: Option<String>,
    date: Option<NaiveDate>,
    rate: Option<String>,
}

impl ExchangeRatesSetReferenceRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExchangeRatesSetReferenceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](ExchangeRatesSetReferenceRequestBuilder::currency)
    /// - [`date`](ExchangeRatesSetReferenceRequestBuilder::date)
    /// - [`rate`](ExchangeRatesSetReferenceRequestBuilder::rate)
    pub fn build(self) -> Result<ExchangeRatesSetReferenceRequest, BuildError> {
        Ok(ExchangeRatesSetReferenceRequest {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
        })
    }
}
