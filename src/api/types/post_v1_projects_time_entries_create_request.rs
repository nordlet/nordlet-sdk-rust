pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsTimeEntriesCreateRequest {
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default)]
    pub date: String,
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

impl PostV1ProjectsTimeEntriesCreateRequest {
    pub fn builder() -> PostV1ProjectsTimeEntriesCreateRequestBuilder {
        <PostV1ProjectsTimeEntriesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesCreateRequestBuilder {
    project_id: Option<String>,
    employee_id: Option<String>,
    date: Option<String>,
    hours: Option<String>,
    description: Option<String>,
    billable: Option<bool>,
    hourly_rate: Option<String>,
}

impl PostV1ProjectsTimeEntriesCreateRequestBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](PostV1ProjectsTimeEntriesCreateRequestBuilder::project_id)
    /// - [`date`](PostV1ProjectsTimeEntriesCreateRequestBuilder::date)
    /// - [`hours`](PostV1ProjectsTimeEntriesCreateRequestBuilder::hours)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesCreateRequest, BuildError> {
        Ok(PostV1ProjectsTimeEntriesCreateRequest {
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
