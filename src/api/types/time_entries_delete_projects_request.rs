pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimeEntriesDeleteProjectsRequest {
    #[serde(default)]
    pub id: String,
}

impl TimeEntriesDeleteProjectsRequest {
    pub fn builder() -> TimeEntriesDeleteProjectsRequestBuilder {
        <TimeEntriesDeleteProjectsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeEntriesDeleteProjectsRequestBuilder {
    id: Option<String>,
}

impl TimeEntriesDeleteProjectsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimeEntriesDeleteProjectsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimeEntriesDeleteProjectsRequestBuilder::id)
    pub fn build(self) -> Result<TimeEntriesDeleteProjectsRequest, BuildError> {
        Ok(TimeEntriesDeleteProjectsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
