pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrLeaveBalancesSetResponse {
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

impl PostV1HrLeaveBalancesSetResponse {
    pub fn builder() -> PostV1HrLeaveBalancesSetResponseBuilder {
        <PostV1HrLeaveBalancesSetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrLeaveBalancesSetResponseBuilder {
    employee_id: Option<String>,
    year: Option<i64>,
    entitled_days: Option<String>,
    used_days: Option<String>,
    remaining_days: Option<String>,
}

impl PostV1HrLeaveBalancesSetResponseBuilder {
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

    /// Consumes the builder and constructs a [`PostV1HrLeaveBalancesSetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1HrLeaveBalancesSetResponseBuilder::employee_id)
    /// - [`year`](PostV1HrLeaveBalancesSetResponseBuilder::year)
    /// - [`entitled_days`](PostV1HrLeaveBalancesSetResponseBuilder::entitled_days)
    /// - [`used_days`](PostV1HrLeaveBalancesSetResponseBuilder::used_days)
    /// - [`remaining_days`](PostV1HrLeaveBalancesSetResponseBuilder::remaining_days)
    pub fn build(self) -> Result<PostV1HrLeaveBalancesSetResponse, BuildError> {
        Ok(PostV1HrLeaveBalancesSetResponse {
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
