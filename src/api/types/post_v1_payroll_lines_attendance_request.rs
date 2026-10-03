pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollLinesAttendanceRequest {
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

impl PostV1PayrollLinesAttendanceRequest {
    pub fn builder() -> PostV1PayrollLinesAttendanceRequestBuilder {
        <PostV1PayrollLinesAttendanceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollLinesAttendanceRequestBuilder {
    id: Option<String>,
    days_worked: Option<String>,
    hours_worked: Option<String>,
    registered_days: Option<String>,
    average_hourly_earnings: Option<String>,
}

impl PostV1PayrollLinesAttendanceRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PayrollLinesAttendanceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollLinesAttendanceRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PayrollLinesAttendanceRequest, BuildError> {
        Ok(PostV1PayrollLinesAttendanceRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            days_worked: self.days_worked,
            hours_worked: self.hours_worked,
            registered_days: self.registered_days,
            average_hourly_earnings: self.average_hourly_earnings,
        })
    }
}
