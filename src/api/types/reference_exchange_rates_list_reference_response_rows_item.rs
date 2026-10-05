pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesListReferenceResponseRowsItem {
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    pub currency_code: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub rate: String,
}

impl ExchangeRatesListReferenceResponseRowsItem {
    pub fn builder() -> ExchangeRatesListReferenceResponseRowsItemBuilder {
        <ExchangeRatesListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesListReferenceResponseRowsItemBuilder {
    currency_code: Option<String>,
    date: Option<NaiveDate>,
    rate: Option<String>,
}

impl ExchangeRatesListReferenceResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`ExchangeRatesListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency_code`](ExchangeRatesListReferenceResponseRowsItemBuilder::currency_code)
    /// - [`date`](ExchangeRatesListReferenceResponseRowsItemBuilder::date)
    /// - [`rate`](ExchangeRatesListReferenceResponseRowsItemBuilder::rate)
    pub fn build(self) -> Result<ExchangeRatesListReferenceResponseRowsItem, BuildError> {
        Ok(ExchangeRatesListReferenceResponseRowsItem {
            currency_code: self
                .currency_code
                .ok_or_else(|| BuildError::missing_field("currency_code"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
        })
    }
}
