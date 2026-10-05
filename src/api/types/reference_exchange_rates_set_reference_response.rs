pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesSetReferenceResponse {
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    pub currency_code: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub rate: String,
}

impl ExchangeRatesSetReferenceResponse {
    pub fn builder() -> ExchangeRatesSetReferenceResponseBuilder {
        <ExchangeRatesSetReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesSetReferenceResponseBuilder {
    currency_code: Option<String>,
    date: Option<NaiveDate>,
    rate: Option<String>,
}

impl ExchangeRatesSetReferenceResponseBuilder {
    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
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

    /// Consumes the builder and constructs a [`ExchangeRatesSetReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency_code`](ExchangeRatesSetReferenceResponseBuilder::currency_code)
    /// - [`date`](ExchangeRatesSetReferenceResponseBuilder::date)
    /// - [`rate`](ExchangeRatesSetReferenceResponseBuilder::rate)
    pub fn build(self) -> Result<ExchangeRatesSetReferenceResponse, BuildError> {
        Ok(ExchangeRatesSetReferenceResponse {
            currency_code: self
                .currency_code
                .ok_or_else(|| BuildError::missing_field("currency_code"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
        })
    }
}
