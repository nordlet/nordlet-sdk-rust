pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIvazGenerateResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default)]
    pub counts: PostV1DeclarationsLtIvazGenerateResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub xml: String,
}

impl PostV1DeclarationsLtIvazGenerateResponse {
    pub fn builder() -> PostV1DeclarationsLtIvazGenerateResponseBuilder {
        <PostV1DeclarationsLtIvazGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIvazGenerateResponseBuilder {
    file_name: Option<String>,
    file_id: Option<String>,
    counts: Option<PostV1DeclarationsLtIvazGenerateResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    xml: Option<String>,
}

impl PostV1DeclarationsLtIvazGenerateResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn counts(mut self, value: PostV1DeclarationsLtIvazGenerateResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIvazGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsLtIvazGenerateResponseBuilder::file_name)
    /// - [`counts`](PostV1DeclarationsLtIvazGenerateResponseBuilder::counts)
    /// - [`warnings`](PostV1DeclarationsLtIvazGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtIvazGenerateResponseBuilder::notes)
    /// - [`xml`](PostV1DeclarationsLtIvazGenerateResponseBuilder::xml)
    pub fn build(self) -> Result<PostV1DeclarationsLtIvazGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsLtIvazGenerateResponse {
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
