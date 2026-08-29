pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsUploadRequest {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    /// Base64-encoded scan, photo or PDF of the supplier document
    #[serde(default)]
    pub content: String,
}

impl PostV1CaptureDocumentsUploadRequest {
    pub fn builder() -> PostV1CaptureDocumentsUploadRequestBuilder {
        <PostV1CaptureDocumentsUploadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsUploadRequestBuilder {
    file_name: Option<String>,
    mime_type: Option<String>,
    content: Option<String>,
}

impl PostV1CaptureDocumentsUploadRequestBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsUploadRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1CaptureDocumentsUploadRequestBuilder::file_name)
    /// - [`mime_type`](PostV1CaptureDocumentsUploadRequestBuilder::mime_type)
    /// - [`content`](PostV1CaptureDocumentsUploadRequestBuilder::content)
    pub fn build(self) -> Result<PostV1CaptureDocumentsUploadRequest, BuildError> {
        Ok(PostV1CaptureDocumentsUploadRequest {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| BuildError::missing_field("mime_type"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
