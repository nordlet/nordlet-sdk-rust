pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarDownloadResponse {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1CalendarDownloadResponse {
    pub fn builder() -> PostV1CalendarDownloadResponseBuilder {
        <PostV1CalendarDownloadResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarDownloadResponseBuilder {
    key: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    variant: Option<String>,
    content: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1CalendarDownloadResponseBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
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

    pub fn variant(mut self, value: impl Into<String>) -> Self {
        self.variant = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CalendarDownloadResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1CalendarDownloadResponseBuilder::key)
    /// - [`file_name`](PostV1CalendarDownloadResponseBuilder::file_name)
    /// - [`mime_type`](PostV1CalendarDownloadResponseBuilder::mime_type)
    /// - [`content`](PostV1CalendarDownloadResponseBuilder::content)
    /// - [`warnings`](PostV1CalendarDownloadResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1CalendarDownloadResponse, BuildError> {
        Ok(PostV1CalendarDownloadResponse {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| BuildError::missing_field("mime_type"))?,
            variant: self.variant,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
