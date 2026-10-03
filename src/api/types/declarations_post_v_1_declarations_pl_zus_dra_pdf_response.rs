pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlZusDraPdfResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1DeclarationsPlZusDraPdfResponse {
    pub fn builder() -> PostV1DeclarationsPlZusDraPdfResponseBuilder {
        <PostV1DeclarationsPlZusDraPdfResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlZusDraPdfResponseBuilder {
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
    source: Option<String>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PostV1DeclarationsPlZusDraPdfResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlZusDraPdfResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsPlZusDraPdfResponseBuilder::file_name)
    /// - [`content_type`](PostV1DeclarationsPlZusDraPdfResponseBuilder::content_type)
    /// - [`data`](PostV1DeclarationsPlZusDraPdfResponseBuilder::data)
    /// - [`source`](PostV1DeclarationsPlZusDraPdfResponseBuilder::source)
    /// - [`warnings`](PostV1DeclarationsPlZusDraPdfResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsPlZusDraPdfResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1DeclarationsPlZusDraPdfResponse, BuildError> {
        Ok(PostV1DeclarationsPlZusDraPdfResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
