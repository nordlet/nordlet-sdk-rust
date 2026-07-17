pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FilesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1FilesGetRequest {
    pub fn builder() -> PostV1FilesGetRequestBuilder {
        <PostV1FilesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FilesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1FilesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FilesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1FilesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1FilesGetRequest, BuildError> {
        Ok(PostV1FilesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
