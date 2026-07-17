pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIsafGenerateResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub counts: PostV1DeclarationsLtIsafGenerateResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub xml: String,
}

impl PostV1DeclarationsLtIsafGenerateResponse {
    pub fn builder() -> PostV1DeclarationsLtIsafGenerateResponseBuilder {
        <PostV1DeclarationsLtIsafGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIsafGenerateResponseBuilder {
    file_name: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    counts: Option<PostV1DeclarationsLtIsafGenerateResponseCounts>,
    warnings: Option<Vec<String>>,
    xml: Option<String>,
}

impl PostV1DeclarationsLtIsafGenerateResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
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

    pub fn counts(mut self, value: PostV1DeclarationsLtIsafGenerateResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIsafGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsLtIsafGenerateResponseBuilder::file_name)
    /// - [`period_start`](PostV1DeclarationsLtIsafGenerateResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsLtIsafGenerateResponseBuilder::period_end)
    /// - [`counts`](PostV1DeclarationsLtIsafGenerateResponseBuilder::counts)
    /// - [`warnings`](PostV1DeclarationsLtIsafGenerateResponseBuilder::warnings)
    /// - [`xml`](PostV1DeclarationsLtIsafGenerateResponseBuilder::xml)
    pub fn build(self) -> Result<PostV1DeclarationsLtIsafGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsLtIsafGenerateResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
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
