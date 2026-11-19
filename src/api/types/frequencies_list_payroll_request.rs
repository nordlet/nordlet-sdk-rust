pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FrequenciesListPayrollRequest {}

impl FrequenciesListPayrollRequest {
    pub fn builder() -> FrequenciesListPayrollRequestBuilder {
        <FrequenciesListPayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FrequenciesListPayrollRequestBuilder {}

impl FrequenciesListPayrollRequestBuilder {
    /// Consumes the builder and constructs a [`FrequenciesListPayrollRequest`].
    pub fn build(self) -> Result<FrequenciesListPayrollRequest, BuildError> {
        Ok(FrequenciesListPayrollRequest {})
    }
}
