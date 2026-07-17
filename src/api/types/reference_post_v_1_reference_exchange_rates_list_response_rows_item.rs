pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceExchangeRatesListResponseRowsItem {
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    pub currency_code: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub rate: String,
}

impl PostV1ReferenceExchangeRatesListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceExchangeRatesListResponseRowsItemBuilder {
        <PostV1ReferenceExchangeRatesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesListResponseRowsItemBuilder {
    currency_code: Option<String>,
    date: Option<String>,
    rate: Option<String>,
}

impl PostV1ReferenceExchangeRatesListResponseRowsItemBuilder {
    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency_code`](PostV1ReferenceExchangeRatesListResponseRowsItemBuilder::currency_code)
    /// - [`date`](PostV1ReferenceExchangeRatesListResponseRowsItemBuilder::date)
    /// - [`rate`](PostV1ReferenceExchangeRatesListResponseRowsItemBuilder::rate)
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceExchangeRatesListResponseRowsItem {
            currency_code: self
                .currency_code
                .ok_or_else(|| BuildError::missing_field("currency_code"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
        })
    }
}
