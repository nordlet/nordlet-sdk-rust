pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimeEntriesDeleteProjectsResponse {
    #[serde(default)]
    pub id: String,
}

impl TimeEntriesDeleteProjectsResponse {
    pub fn builder() -> TimeEntriesDeleteProjectsResponseBuilder {
        <TimeEntriesDeleteProjectsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeEntriesDeleteProjectsResponseBuilder {
    id: Option<String>,
}

impl TimeEntriesDeleteProjectsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimeEntriesDeleteProjectsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimeEntriesDeleteProjectsResponseBuilder::id)
    pub fn build(self) -> Result<TimeEntriesDeleteProjectsResponse, BuildError> {
        Ok(TimeEntriesDeleteProjectsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
