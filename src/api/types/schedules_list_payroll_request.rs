pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SchedulesListPayrollRequest {}

impl SchedulesListPayrollRequest {
    pub fn builder() -> SchedulesListPayrollRequestBuilder {
        <SchedulesListPayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SchedulesListPayrollRequestBuilder {}

impl SchedulesListPayrollRequestBuilder {
    /// Consumes the builder and constructs a [`SchedulesListPayrollRequest`].
    pub fn build(self) -> Result<SchedulesListPayrollRequest, BuildError> {
        Ok(SchedulesListPayrollRequest {})
    }
}
