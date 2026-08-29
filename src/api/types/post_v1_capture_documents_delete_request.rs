pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CaptureDocumentsDeleteRequest {
    pub fn builder() -> PostV1CaptureDocumentsDeleteRequestBuilder {
        <PostV1CaptureDocumentsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1CaptureDocumentsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CaptureDocumentsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CaptureDocumentsDeleteRequest, BuildError> {
        Ok(PostV1CaptureDocumentsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
