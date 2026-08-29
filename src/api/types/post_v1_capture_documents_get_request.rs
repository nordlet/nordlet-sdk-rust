pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CaptureDocumentsGetRequest {
    pub fn builder() -> PostV1CaptureDocumentsGetRequestBuilder {
        <PostV1CaptureDocumentsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1CaptureDocumentsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CaptureDocumentsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CaptureDocumentsGetRequest, BuildError> {
        Ok(PostV1CaptureDocumentsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
