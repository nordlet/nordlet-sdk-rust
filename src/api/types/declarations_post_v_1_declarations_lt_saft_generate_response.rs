pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSaftGenerateResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub counts: PostV1DeclarationsLtSaftGenerateResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub xml: String,
}

impl PostV1DeclarationsLtSaftGenerateResponse {
    pub fn builder() -> PostV1DeclarationsLtSaftGenerateResponseBuilder {
        <PostV1DeclarationsLtSaftGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSaftGenerateResponseBuilder {
    file_name: Option<String>,
    file_id: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    counts: Option<PostV1DeclarationsLtSaftGenerateResponseCounts>,
    warnings: Option<Vec<String>>,
    xml: Option<String>,
}

impl PostV1DeclarationsLtSaftGenerateResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn period_start(mut self, value: impl Into<String>) -> Self {
        self.period_start = Some(value.into());
        self
    }

    pub fn period_end(mut self, value: impl Into<String>) -> Self {
        self.period_end = Some(value.into());
        self
    }

    pub fn counts(mut self, value: PostV1DeclarationsLtSaftGenerateResponseCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSaftGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsLtSaftGenerateResponseBuilder::file_name)
    /// - [`period_start`](PostV1DeclarationsLtSaftGenerateResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsLtSaftGenerateResponseBuilder::period_end)
    /// - [`counts`](PostV1DeclarationsLtSaftGenerateResponseBuilder::counts)
    /// - [`warnings`](PostV1DeclarationsLtSaftGenerateResponseBuilder::warnings)
    /// - [`xml`](PostV1DeclarationsLtSaftGenerateResponseBuilder::xml)
    pub fn build(self) -> Result<PostV1DeclarationsLtSaftGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsLtSaftGenerateResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file_id: self.file_id,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
        })
    }
}
