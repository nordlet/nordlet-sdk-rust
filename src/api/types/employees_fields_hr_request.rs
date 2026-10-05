pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesFieldsHrRequest {}

impl EmployeesFieldsHrRequest {
    pub fn builder() -> EmployeesFieldsHrRequestBuilder {
        <EmployeesFieldsHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesFieldsHrRequestBuilder {}

impl EmployeesFieldsHrRequestBuilder {
    /// Consumes the builder and constructs a [`EmployeesFieldsHrRequest`].
    pub fn build(self) -> Result<EmployeesFieldsHrRequest, BuildError> {
        Ok(EmployeesFieldsHrRequest {})
    }
}
