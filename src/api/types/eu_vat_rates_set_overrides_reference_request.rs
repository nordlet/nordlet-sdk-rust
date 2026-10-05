pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuVatRatesSetOverridesReferenceRequest {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub rates: Vec<EuVatRatesSetOverridesReferenceRequestRatesItem>,
}

impl EuVatRatesSetOverridesReferenceRequest {
    pub fn builder() -> EuVatRatesSetOverridesReferenceRequestBuilder {
        <EuVatRatesSetOverridesReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatRatesSetOverridesReferenceRequestBuilder {
    country_code: Option<String>,
    rates: Option<Vec<EuVatRatesSetOverridesReferenceRequestRatesItem>>,
}

impl EuVatRatesSetOverridesReferenceRequestBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn rates(mut self, value: Vec<EuVatRatesSetOverridesReferenceRequestRatesItem>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuVatRatesSetOverridesReferenceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuVatRatesSetOverridesReferenceRequestBuilder::country_code)
    /// - [`rates`](EuVatRatesSetOverridesReferenceRequestBuilder::rates)
    pub fn build(self) -> Result<EuVatRatesSetOverridesReferenceRequest, BuildError> {
        Ok(EuVatRatesSetOverridesReferenceRequest {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
        })
    }
}
