pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrLeaveBalancesListRequest {
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}

impl PostV1HrLeaveBalancesListRequest {
    pub fn builder() -> PostV1HrLeaveBalancesListRequestBuilder {
        <PostV1HrLeaveBalancesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrLeaveBalancesListRequestBuilder {
    employee_id: Option<String>,
    year: Option<i64>,
}

impl PostV1HrLeaveBalancesListRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrLeaveBalancesListRequest`].
    pub fn build(self) -> Result<PostV1HrLeaveBalancesListRequest, BuildError> {
        Ok(PostV1HrLeaveBalancesListRequest {
            employee_id: self.employee_id,
            year: self.year,
        })
    }
}
