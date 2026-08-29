pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsExtractRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CaptureDocumentsExtractRequest {
    pub fn builder() -> PostV1CaptureDocumentsExtractRequestBuilder {
        <PostV1CaptureDocumentsExtractRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsExtractRequestBuilder {
    id: Option<String>,
}

impl PostV1CaptureDocumentsExtractRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsExtractRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CaptureDocumentsExtractRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CaptureDocumentsExtractRequest, BuildError> {
        Ok(PostV1CaptureDocumentsExtractRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
