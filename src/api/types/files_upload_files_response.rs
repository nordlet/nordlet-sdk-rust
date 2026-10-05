pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UploadFilesResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub entity: String,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
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
    #[serde(rename = "storageKey")]
    #[serde(default)]
    pub storage_key: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl UploadFilesResponse {
    pub fn builder() -> UploadFilesResponseBuilder {
        <UploadFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadFilesResponseBuilder {
    id: Option<String>,
    entity: Option<String>,
    entity_id: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
    sha256: Option<String>,
    storage_key: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl UploadFilesResponseBuilder {
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

    pub fn storage_key(mut self, value: impl Into<String>) -> Self {
        self.storage_key = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UploadFilesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UploadFilesResponseBuilder::id)
    /// - [`entity`](UploadFilesResponseBuilder::entity)
    /// - [`file_name`](UploadFilesResponseBuilder::file_name)
    /// - [`mime_type`](UploadFilesResponseBuilder::mime_type)
    /// - [`size_bytes`](UploadFilesResponseBuilder::size_bytes)
    /// - [`sha256`](UploadFilesResponseBuilder::sha256)
    /// - [`storage_key`](UploadFilesResponseBuilder::storage_key)
    /// - [`created_at`](UploadFilesResponseBuilder::created_at)
    pub fn build(self) -> Result<UploadFilesResponse, BuildError> {
        Ok(UploadFilesResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            entity: self
                .entity
                .ok_or_else(|| BuildError::missing_field("entity"))?,
            entity_id: self.entity_id,
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
            storage_key: self
                .storage_key
                .ok_or_else(|| BuildError::missing_field("storage_key"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
