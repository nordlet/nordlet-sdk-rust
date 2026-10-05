pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetFilesRequest {
    #[serde(default)]
    pub id: String,
}

impl GetFilesRequest {
    pub fn builder() -> GetFilesRequestBuilder {
        <GetFilesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetFilesRequestBuilder {
    id: Option<String>,
}

impl GetFilesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetFilesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetFilesRequestBuilder::id)
    pub fn build(self) -> Result<GetFilesRequest, BuildError> {
        Ok(GetFilesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
