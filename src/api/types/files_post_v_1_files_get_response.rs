pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FilesGetResponse {
    #[serde(default)]
    pub id: String,
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
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    pub size_bytes: i64,
    #[serde(default)]
    pub sha256: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub content: String,
}

impl PostV1FilesGetResponse {
    pub fn builder() -> PostV1FilesGetResponseBuilder {
        <PostV1FilesGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FilesGetResponseBuilder {
    id: Option<String>,
    entity: Option<String>,
    entity_id: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
    sha256: Option<String>,
    created_at: Option<String>,
    content: Option<String>,
}

impl PostV1FilesGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn sha256(mut self, value: impl Into<String>) -> Self {
        self.sha256 = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FilesGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1FilesGetResponseBuilder::id)
    /// - [`entity`](PostV1FilesGetResponseBuilder::entity)
    /// - [`entity_id`](PostV1FilesGetResponseBuilder::entity_id)
    /// - [`file_name`](PostV1FilesGetResponseBuilder::file_name)
    /// - [`mime_type`](PostV1FilesGetResponseBuilder::mime_type)
    /// - [`size_bytes`](PostV1FilesGetResponseBuilder::size_bytes)
    /// - [`sha256`](PostV1FilesGetResponseBuilder::sha256)
    /// - [`created_at`](PostV1FilesGetResponseBuilder::created_at)
    /// - [`content`](PostV1FilesGetResponseBuilder::content)
    pub fn build(self) -> Result<PostV1FilesGetResponse, BuildError> {
        Ok(PostV1FilesGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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
            size_bytes: self
                .size_bytes
                .ok_or_else(|| BuildError::missing_field("size_bytes"))?,
            sha256: self
                .sha256
                .ok_or_else(|| BuildError::missing_field("sha256"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
