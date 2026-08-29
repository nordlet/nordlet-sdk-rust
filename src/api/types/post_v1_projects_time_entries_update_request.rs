pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsTimeEntriesUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
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

impl PostV1ProjectsTimeEntriesUpdateRequest {
    pub fn builder() -> PostV1ProjectsTimeEntriesUpdateRequestBuilder {
        <PostV1ProjectsTimeEntriesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesUpdateRequestBuilder {
    id: Option<String>,
    date: Option<String>,
    hours: Option<String>,
    description: Option<String>,
    billable: Option<bool>,
    hourly_rate: Option<String>,
}

impl PostV1ProjectsTimeEntriesUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProjectsTimeEntriesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesUpdateRequest, BuildError> {
        Ok(PostV1ProjectsTimeEntriesUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date,
            hours: self.hours,
            description: self.description,
            billable: self.billable,
            hourly_rate: self.hourly_rate,
        })
    }
}
