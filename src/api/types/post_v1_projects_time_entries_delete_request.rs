pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsTimeEntriesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1ProjectsTimeEntriesDeleteRequest {
    pub fn builder() -> PostV1ProjectsTimeEntriesDeleteRequestBuilder {
        <PostV1ProjectsTimeEntriesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1ProjectsTimeEntriesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProjectsTimeEntriesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesDeleteRequest, BuildError> {
        Ok(PostV1ProjectsTimeEntriesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
