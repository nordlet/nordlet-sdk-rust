pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FilesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1FilesDeleteRequest {
    pub fn builder() -> PostV1FilesDeleteRequestBuilder {
        <PostV1FilesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FilesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1FilesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FilesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1FilesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1FilesDeleteRequest, BuildError> {
        Ok(PostV1FilesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
