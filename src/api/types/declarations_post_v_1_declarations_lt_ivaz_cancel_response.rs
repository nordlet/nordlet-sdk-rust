pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIvazCancelResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default)]
    pub counts: PostV1DeclarationsLtIvazCancelResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub xml: String,
}

impl PostV1DeclarationsLtIvazCancelResponse {
    pub fn builder() -> PostV1DeclarationsLtIvazCancelResponseBuilder {
        <PostV1DeclarationsLtIvazCancelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIvazCancelResponseBuilder {
    file_name: Option<String>,
    file_id: Option<String>,
    counts: Option<PostV1DeclarationsLtIvazCancelResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    xml: Option<String>,
}

impl PostV1DeclarationsLtIvazCancelResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn counts(mut self, value: PostV1DeclarationsLtIvazCancelResponseCounts) -> Self {
        self.counts = Some(value);
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

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIvazCancelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsLtIvazCancelResponseBuilder::file_name)
    /// - [`counts`](PostV1DeclarationsLtIvazCancelResponseBuilder::counts)
    /// - [`warnings`](PostV1DeclarationsLtIvazCancelResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtIvazCancelResponseBuilder::notes)
    /// - [`xml`](PostV1DeclarationsLtIvazCancelResponseBuilder::xml)
    pub fn build(self) -> Result<PostV1DeclarationsLtIvazCancelResponse, BuildError> {
        Ok(PostV1DeclarationsLtIvazCancelResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file_id: self.file_id,
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
        })
    }
}
