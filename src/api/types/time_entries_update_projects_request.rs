pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimeEntriesUpdateProjectsRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<bool>,
    #[serde(rename = "hourlyRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_rate: Option<String>,
}

impl TimeEntriesUpdateProjectsRequest {
    pub fn builder() -> TimeEntriesUpdateProjectsRequestBuilder {
        <TimeEntriesUpdateProjectsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeEntriesUpdateProjectsRequestBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    hours: Option<String>,
    description: Option<String>,
    billable: Option<bool>,
    hourly_rate: Option<String>,
}

impl TimeEntriesUpdateProjectsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`TimeEntriesUpdateProjectsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimeEntriesUpdateProjectsRequestBuilder::id)
    pub fn build(self) -> Result<TimeEntriesUpdateProjectsRequest, BuildError> {
        Ok(TimeEntriesUpdateProjectsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date,
            hours: self.hours,
            description: self.description,
            billable: self.billable,
            hourly_rate: self.hourly_rate,
        })
    }
}
