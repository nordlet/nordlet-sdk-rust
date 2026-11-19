pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FrequenciesListPayrollResponse {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub frequencies: Vec<FrequenciesListPayrollResponseFrequenciesItem>,
}

impl FrequenciesListPayrollResponse {
    pub fn builder() -> FrequenciesListPayrollResponseBuilder {
        <FrequenciesListPayrollResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FrequenciesListPayrollResponseBuilder {
    country_code: Option<String>,
    frequencies: Option<Vec<FrequenciesListPayrollResponseFrequenciesItem>>,
}

impl FrequenciesListPayrollResponseBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn frequencies(
        mut self,
        value: Vec<FrequenciesListPayrollResponseFrequenciesItem>,
    ) -> Self {
        self.frequencies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FrequenciesListPayrollResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](FrequenciesListPayrollResponseBuilder::country_code)
    /// - [`frequencies`](FrequenciesListPayrollResponseBuilder::frequencies)
    pub fn build(self) -> Result<FrequenciesListPayrollResponse, BuildError> {
        Ok(FrequenciesListPayrollResponse {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            frequencies: self
                .frequencies
                .ok_or_else(|| BuildError::missing_field("frequencies"))?,
        })
    }
}
