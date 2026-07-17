pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FilesUploadRequest {
    #[serde(default)]
    pub entity: String,
    #[serde(rename = "entityId")]
    #[serde(default)]
    pub entity_id: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    /// Base64-encoded file content
    #[serde(default)]
    pub content: String,
}

impl PostV1FilesUploadRequest {
    pub fn builder() -> PostV1FilesUploadRequestBuilder {
        <PostV1FilesUploadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FilesUploadRequestBuilder {
    entity: Option<String>,
    entity_id: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    content: Option<String>,
}

impl PostV1FilesUploadRequestBuilder {
    pub fn entity(mut self, value: impl Into<String>) -> Self {
        self.entity = Some(value.into());
        self
    }

    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`PostV1FilesUploadRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity`](PostV1FilesUploadRequestBuilder::entity)
    /// - [`entity_id`](PostV1FilesUploadRequestBuilder::entity_id)
    /// - [`file_name`](PostV1FilesUploadRequestBuilder::file_name)
    /// - [`mime_type`](PostV1FilesUploadRequestBuilder::mime_type)
    /// - [`content`](PostV1FilesUploadRequestBuilder::content)
    pub fn build(self) -> Result<PostV1FilesUploadRequest, BuildError> {
        Ok(PostV1FilesUploadRequest {
            entity: self
                .entity
                .ok_or_else(|| BuildError::missing_field("entity"))?,
            entity_id: self
                .entity_id
                .ok_or_else(|| BuildError::missing_field("entity_id"))?,
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
