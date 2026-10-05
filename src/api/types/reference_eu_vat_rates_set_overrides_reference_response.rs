pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuVatRatesSetOverridesReferenceResponse {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub source: EuVatRatesSetOverridesReferenceResponseSource,
    #[serde(default)]
    pub notice: String,
    #[serde(default)]
    pub rows: Vec<EuVatRatesSetOverridesReferenceResponseRowsItem>,
}

impl EuVatRatesSetOverridesReferenceResponse {
    pub fn builder() -> EuVatRatesSetOverridesReferenceResponseBuilder {
        <EuVatRatesSetOverridesReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatRatesSetOverridesReferenceResponseBuilder {
    country_code: Option<String>,
    source: Option<EuVatRatesSetOverridesReferenceResponseSource>,
    notice: Option<String>,
    rows: Option<Vec<EuVatRatesSetOverridesReferenceResponseRowsItem>>,
}

impl EuVatRatesSetOverridesReferenceResponseBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn source(mut self, value: EuVatRatesSetOverridesReferenceResponseSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn notice(mut self, value: impl Into<String>) -> Self {
        self.notice = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<EuVatRatesSetOverridesReferenceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuVatRatesSetOverridesReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuVatRatesSetOverridesReferenceResponseBuilder::country_code)
    /// - [`source`](EuVatRatesSetOverridesReferenceResponseBuilder::source)
    /// - [`notice`](EuVatRatesSetOverridesReferenceResponseBuilder::notice)
    /// - [`rows`](EuVatRatesSetOverridesReferenceResponseBuilder::rows)
    pub fn build(self) -> Result<EuVatRatesSetOverridesReferenceResponse, BuildError> {
        Ok(EuVatRatesSetOverridesReferenceResponse {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            notice: self
                .notice
                .ok_or_else(|| BuildError::missing_field("notice"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
