pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesSetOverridesRequest {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub rates: Vec<PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem>,
}

impl PostV1ReferenceEuVatRatesSetOverridesRequest {
    pub fn builder() -> PostV1ReferenceEuVatRatesSetOverridesRequestBuilder {
        <PostV1ReferenceEuVatRatesSetOverridesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesSetOverridesRequestBuilder {
    country_code: Option<String>,
    rates: Option<Vec<PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem>>,
}

impl PostV1ReferenceEuVatRatesSetOverridesRequestBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn rates(
        mut self,
        value: Vec<PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesSetOverridesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1ReferenceEuVatRatesSetOverridesRequestBuilder::country_code)
    /// - [`rates`](PostV1ReferenceEuVatRatesSetOverridesRequestBuilder::rates)
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesSetOverridesRequest, BuildError> {
        Ok(PostV1ReferenceEuVatRatesSetOverridesRequest {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
        })
    }
}
