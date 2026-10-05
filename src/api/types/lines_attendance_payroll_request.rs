pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinesAttendancePayrollRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "daysWorked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_worked: Option<String>,
    #[serde(rename = "hoursWorked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_worked: Option<String>,
    #[serde(rename = "registeredDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_days: Option<String>,
    #[serde(rename = "averageHourlyEarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_hourly_earnings: Option<String>,
}

impl LinesAttendancePayrollRequest {
    pub fn builder() -> LinesAttendancePayrollRequestBuilder {
        <LinesAttendancePayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinesAttendancePayrollRequestBuilder {
    id: Option<String>,
    days_worked: Option<String>,
    hours_worked: Option<String>,
    registered_days: Option<String>,
    average_hourly_earnings: Option<String>,
}

impl LinesAttendancePayrollRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn days_worked(mut self, value: impl Into<String>) -> Self {
        self.days_worked = Some(value.into());
        self
    }

    pub fn hours_worked(mut self, value: impl Into<String>) -> Self {
        self.hours_worked = Some(value.into());
        self
    }

    pub fn registered_days(mut self, value: impl Into<String>) -> Self {
        self.registered_days = Some(value.into());
        self
    }

    pub fn average_hourly_earnings(mut self, value: impl Into<String>) -> Self {
        self.average_hourly_earnings = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinesAttendancePayrollRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LinesAttendancePayrollRequestBuilder::id)
    pub fn build(self) -> Result<LinesAttendancePayrollRequest, BuildError> {
        Ok(LinesAttendancePayrollRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            days_worked: self.days_worked,
            hours_worked: self.hours_worked,
            registered_days: self.registered_days,
            average_hourly_earnings: self.average_hourly_earnings,
        })
    }
}
