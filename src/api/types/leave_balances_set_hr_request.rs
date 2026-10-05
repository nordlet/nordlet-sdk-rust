pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaveBalancesSetHrRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "entitledDays")]
    #[serde(default)]
    pub entitled_days: String,
    #[serde(rename = "usedDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_days: Option<String>,
}

impl LeaveBalancesSetHrRequest {
    pub fn builder() -> LeaveBalancesSetHrRequestBuilder {
        <LeaveBalancesSetHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaveBalancesSetHrRequestBuilder {
    employee_id: Option<String>,
    year: Option<i64>,
    entitled_days: Option<String>,
    used_days: Option<String>,
}

impl LeaveBalancesSetHrRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn entitled_days(mut self, value: impl Into<String>) -> Self {
        self.entitled_days = Some(value.into());
        self
    }

    pub fn used_days(mut self, value: impl Into<String>) -> Self {
        self.used_days = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeaveBalancesSetHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](LeaveBalancesSetHrRequestBuilder::employee_id)
    /// - [`year`](LeaveBalancesSetHrRequestBuilder::year)
    /// - [`entitled_days`](LeaveBalancesSetHrRequestBuilder::entitled_days)
    pub fn build(self) -> Result<LeaveBalancesSetHrRequest, BuildError> {
        Ok(LeaveBalancesSetHrRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            entitled_days: self
                .entitled_days
                .ok_or_else(|| BuildError::missing_field("entitled_days"))?,
            used_days: self.used_days,
        })
    }
}
