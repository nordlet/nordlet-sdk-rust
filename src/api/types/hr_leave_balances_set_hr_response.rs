pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaveBalancesSetHrResponse {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "entitledDays")]
    #[serde(default)]
    pub entitled_days: String,
    #[serde(rename = "usedDays")]
    #[serde(default)]
    pub used_days: String,
    #[serde(rename = "remainingDays")]
    #[serde(default)]
    pub remaining_days: String,
}

impl LeaveBalancesSetHrResponse {
    pub fn builder() -> LeaveBalancesSetHrResponseBuilder {
        <LeaveBalancesSetHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaveBalancesSetHrResponseBuilder {
    employee_id: Option<String>,
    year: Option<i64>,
    entitled_days: Option<String>,
    used_days: Option<String>,
    remaining_days: Option<String>,
}

impl LeaveBalancesSetHrResponseBuilder {
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

    pub fn remaining_days(mut self, value: impl Into<String>) -> Self {
        self.remaining_days = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeaveBalancesSetHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](LeaveBalancesSetHrResponseBuilder::employee_id)
    /// - [`year`](LeaveBalancesSetHrResponseBuilder::year)
    /// - [`entitled_days`](LeaveBalancesSetHrResponseBuilder::entitled_days)
    /// - [`used_days`](LeaveBalancesSetHrResponseBuilder::used_days)
    /// - [`remaining_days`](LeaveBalancesSetHrResponseBuilder::remaining_days)
    pub fn build(self) -> Result<LeaveBalancesSetHrResponse, BuildError> {
        Ok(LeaveBalancesSetHrResponse {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            entitled_days: self
                .entitled_days
                .ok_or_else(|| BuildError::missing_field("entitled_days"))?,
            used_days: self
                .used_days
                .ok_or_else(|| BuildError::missing_field("used_days"))?,
            remaining_days: self
                .remaining_days
                .ok_or_else(|| BuildError::missing_field("remaining_days"))?,
        })
    }
}
