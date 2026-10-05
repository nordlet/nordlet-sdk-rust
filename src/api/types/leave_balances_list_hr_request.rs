pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaveBalancesListHrRequest {
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}

impl LeaveBalancesListHrRequest {
    pub fn builder() -> LeaveBalancesListHrRequestBuilder {
        <LeaveBalancesListHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaveBalancesListHrRequestBuilder {
    employee_id: Option<String>,
    year: Option<i64>,
}

impl LeaveBalancesListHrRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LeaveBalancesListHrRequest`].
    pub fn build(self) -> Result<LeaveBalancesListHrRequest, BuildError> {
        Ok(LeaveBalancesListHrRequest {
            employee_id: self.employee_id,
            year: self.year,
        })
    }
}
