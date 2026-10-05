pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimeEntriesCreateProjectsRequest {
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub hours: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<bool>,
    #[serde(rename = "hourlyRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_rate: Option<String>,
}

impl TimeEntriesCreateProjectsRequest {
    pub fn builder() -> TimeEntriesCreateProjectsRequestBuilder {
        <TimeEntriesCreateProjectsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeEntriesCreateProjectsRequestBuilder {
    project_id: Option<String>,
    employee_id: Option<String>,
    date: Option<NaiveDate>,
    hours: Option<String>,
    description: Option<String>,
    billable: Option<bool>,
    hourly_rate: Option<String>,
}

impl TimeEntriesCreateProjectsRequestBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn billable(mut self, value: bool) -> Self {
        self.billable = Some(value);
        self
    }

    pub fn hourly_rate(mut self, value: impl Into<String>) -> Self {
        self.hourly_rate = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimeEntriesCreateProjectsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](TimeEntriesCreateProjectsRequestBuilder::project_id)
    /// - [`date`](TimeEntriesCreateProjectsRequestBuilder::date)
    /// - [`hours`](TimeEntriesCreateProjectsRequestBuilder::hours)
    pub fn build(self) -> Result<TimeEntriesCreateProjectsRequest, BuildError> {
        Ok(TimeEntriesCreateProjectsRequest {
            project_id: self
                .project_id
                .ok_or_else(|| BuildError::missing_field("project_id"))?,
            employee_id: self.employee_id,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            hours: self
                .hours
                .ok_or_else(|| BuildError::missing_field("hours"))?,
            description: self.description,
            billable: self.billable,
            hourly_rate: self.hourly_rate,
        })
    }
}
