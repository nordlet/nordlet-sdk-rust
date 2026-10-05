pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProjectsRequest {
    #[serde(default)]
    pub id: String,
}

impl GetProjectsRequest {
    pub fn builder() -> GetProjectsRequestBuilder {
        <GetProjectsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProjectsRequestBuilder {
    id: Option<String>,
}

impl GetProjectsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetProjectsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetProjectsRequestBuilder::id)
    pub fn build(self) -> Result<GetProjectsRequest, BuildError> {
        Ok(GetProjectsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
