pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesAttachmentsListRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
}

impl PostV1HrEmployeesAttachmentsListRequest {
    pub fn builder() -> PostV1HrEmployeesAttachmentsListRequestBuilder {
        <PostV1HrEmployeesAttachmentsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesAttachmentsListRequestBuilder {
    employee_id: Option<String>,
}

impl PostV1HrEmployeesAttachmentsListRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesAttachmentsListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1HrEmployeesAttachmentsListRequestBuilder::employee_id)
    pub fn build(self) -> Result<PostV1HrEmployeesAttachmentsListRequest, BuildError> {
        Ok(PostV1HrEmployeesAttachmentsListRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
        })
    }
}
