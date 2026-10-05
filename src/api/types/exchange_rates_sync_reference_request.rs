pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExchangeRatesSyncReferenceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl ExchangeRatesSyncReferenceRequest {
    pub fn builder() -> ExchangeRatesSyncReferenceRequestBuilder {
        <ExchangeRatesSyncReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeRatesSyncReferenceRequestBuilder {
    date: Option<NaiveDate>,
}

impl ExchangeRatesSyncReferenceRequestBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExchangeRatesSyncReferenceRequest`].
    pub fn build(self) -> Result<ExchangeRatesSyncReferenceRequest, BuildError> {
        Ok(ExchangeRatesSyncReferenceRequest { date: self.date })
    }
}
