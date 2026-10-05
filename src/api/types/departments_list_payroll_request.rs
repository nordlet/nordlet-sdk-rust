pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DepartmentsListPayrollRequest {}

impl DepartmentsListPayrollRequest {
    pub fn builder() -> DepartmentsListPayrollRequestBuilder {
        <DepartmentsListPayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepartmentsListPayrollRequestBuilder {}

impl DepartmentsListPayrollRequestBuilder {
    /// Consumes the builder and constructs a [`DepartmentsListPayrollRequest`].
    pub fn build(self) -> Result<DepartmentsListPayrollRequest, BuildError> {
        Ok(DepartmentsListPayrollRequest {})
    }
}
