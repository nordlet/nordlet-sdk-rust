pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesOverridesListReferenceResponseRowsItem {
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    pub currency_code: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub rate: String,
}

impl ExchangeRatesOverridesListReferenceResponseRowsItem {
    pub fn builder() -> ExchangeRatesOverridesListReferenceResponseRowsItemBuilder {
        <ExchangeRatesOverridesListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesOverridesListReferenceResponseRowsItemBuilder {
    currency_code: Option<String>,
    date: Option<NaiveDate>,
    rate: Option<String>,
}

impl ExchangeRatesOverridesListReferenceResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`ExchangeRatesOverridesListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency_code`](ExchangeRatesOverridesListReferenceResponseRowsItemBuilder::currency_code)
    /// - [`date`](ExchangeRatesOverridesListReferenceResponseRowsItemBuilder::date)
    /// - [`rate`](ExchangeRatesOverridesListReferenceResponseRowsItemBuilder::rate)
    pub fn build(self) -> Result<ExchangeRatesOverridesListReferenceResponseRowsItem, BuildError> {
        Ok(ExchangeRatesOverridesListReferenceResponseRowsItem {
            currency_code: self
                .currency_code
                .ok_or_else(|| BuildError::missing_field("currency_code"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
        })
    }
}
