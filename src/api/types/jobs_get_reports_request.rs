pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JobsGetReportsRequest {
    #[serde(default)]
    pub id: String,
}

impl JobsGetReportsRequest {
    pub fn builder() -> JobsGetReportsRequestBuilder {
        <JobsGetReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsGetReportsRequestBuilder {
    id: Option<String>,
}

impl JobsGetReportsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JobsGetReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](JobsGetReportsRequestBuilder::id)
    pub fn build(self) -> Result<JobsGetReportsRequest, BuildError> {
        Ok(JobsGetReportsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
