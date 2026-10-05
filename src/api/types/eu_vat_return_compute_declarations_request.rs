pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuVatReturnComputeDeclarationsRequest {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<i64>,
}

impl EuVatReturnComputeDeclarationsRequest {
    pub fn builder() -> EuVatReturnComputeDeclarationsRequestBuilder {
        <EuVatReturnComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatReturnComputeDeclarationsRequestBuilder {
    country_code: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    months: Option<i64>,
}

impl EuVatReturnComputeDeclarationsRequestBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn months(mut self, value: i64) -> Self {
        self.months = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuVatReturnComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuVatReturnComputeDeclarationsRequestBuilder::country_code)
    /// - [`year`](EuVatReturnComputeDeclarationsRequestBuilder::year)
    /// - [`month`](EuVatReturnComputeDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<EuVatReturnComputeDeclarationsRequest, BuildError> {
        Ok(EuVatReturnComputeDeclarationsRequest {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            months: self.months,
        })
    }
}
