pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuVatRatesListReferenceRequest {
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl EuVatRatesListReferenceRequest {
    pub fn builder() -> EuVatRatesListReferenceRequestBuilder {
        <EuVatRatesListReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatRatesListReferenceRequestBuilder {
    country_code: Option<String>,
    date: Option<NaiveDate>,
}

impl EuVatRatesListReferenceRequestBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuVatRatesListReferenceRequest`].
    pub fn build(self) -> Result<EuVatRatesListReferenceRequest, BuildError> {
        Ok(EuVatRatesListReferenceRequest {
            country_code: self.country_code,
            date: self.date,
        })
    }
}
