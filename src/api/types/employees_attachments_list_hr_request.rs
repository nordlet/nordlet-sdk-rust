pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesAttachmentsListHrRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
}

impl EmployeesAttachmentsListHrRequest {
    pub fn builder() -> EmployeesAttachmentsListHrRequestBuilder {
        <EmployeesAttachmentsListHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesAttachmentsListHrRequestBuilder {
    employee_id: Option<String>,
}

impl EmployeesAttachmentsListHrRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesAttachmentsListHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](EmployeesAttachmentsListHrRequestBuilder::employee_id)
    pub fn build(self) -> Result<EmployeesAttachmentsListHrRequest, BuildError> {
        Ok(EmployeesAttachmentsListHrRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
        })
    }
}
