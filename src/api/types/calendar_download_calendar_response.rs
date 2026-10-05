pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DownloadCalendarResponse {
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

impl DownloadCalendarResponse {
    pub fn builder() -> DownloadCalendarResponseBuilder {
        <DownloadCalendarResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DownloadCalendarResponseBuilder {
    key: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    variant: Option<String>,
    content: Option<String>,
    warnings: Option<Vec<String>>,
}

impl DownloadCalendarResponseBuilder {
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

    /// Consumes the builder and constructs a [`DownloadCalendarResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](DownloadCalendarResponseBuilder::key)
    /// - [`file_name`](DownloadCalendarResponseBuilder::file_name)
    /// - [`mime_type`](DownloadCalendarResponseBuilder::mime_type)
    /// - [`content`](DownloadCalendarResponseBuilder::content)
    /// - [`warnings`](DownloadCalendarResponseBuilder::warnings)
    pub fn build(self) -> Result<DownloadCalendarResponse, BuildError> {
        Ok(DownloadCalendarResponse {
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
