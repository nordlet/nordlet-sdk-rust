pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesSyncReferenceResponse {
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "baseCurrency")]
    #[serde(default)]
    pub base_currency: String,
    #[serde(default)]
    pub imported: i64,
}

impl ExchangeRatesSyncReferenceResponse {
    pub fn builder() -> ExchangeRatesSyncReferenceResponseBuilder {
        <ExchangeRatesSyncReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesSyncReferenceResponseBuilder {
    date: Option<NaiveDate>,
    base_currency: Option<String>,
    imported: Option<i64>,
}

impl ExchangeRatesSyncReferenceResponseBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn base_currency(mut self, value: impl Into<String>) -> Self {
        self.base_currency = Some(value.into());
        self
    }

    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExchangeRatesSyncReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](ExchangeRatesSyncReferenceResponseBuilder::date)
    /// - [`base_currency`](ExchangeRatesSyncReferenceResponseBuilder::base_currency)
    /// - [`imported`](ExchangeRatesSyncReferenceResponseBuilder::imported)
    pub fn build(self) -> Result<ExchangeRatesSyncReferenceResponse, BuildError> {
        Ok(ExchangeRatesSyncReferenceResponse {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
        })
    }
}
