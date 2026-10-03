pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkMagGenerateResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
    #[serde(rename = "warehouseCode")]
    #[serde(default)]
    pub warehouse_code: String,
    #[serde(default)]
    pub counts: PostV1DeclarationsPlJpkMagGenerateResponseCounts,
}

impl PostV1DeclarationsPlJpkMagGenerateResponse {
    pub fn builder() -> PostV1DeclarationsPlJpkMagGenerateResponseBuilder {
        <PostV1DeclarationsPlJpkMagGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkMagGenerateResponseBuilder {
    file_name: Option<String>,
    xml: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
    warehouse_code: Option<String>,
    counts: Option<PostV1DeclarationsPlJpkMagGenerateResponseCounts>,
}

impl PostV1DeclarationsPlJpkMagGenerateResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
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

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn warehouse_code(mut self, value: impl Into<String>) -> Self {
        self.warehouse_code = Some(value.into());
        self
    }

    pub fn counts(mut self, value: PostV1DeclarationsPlJpkMagGenerateResponseCounts) -> Self {
        self.counts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkMagGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsPlJpkMagGenerateResponseBuilder::file_name)
    /// - [`xml`](PostV1DeclarationsPlJpkMagGenerateResponseBuilder::xml)
    /// - [`period_start`](PostV1DeclarationsPlJpkMagGenerateResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsPlJpkMagGenerateResponseBuilder::period_end)
    /// - [`warnings`](PostV1DeclarationsPlJpkMagGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsPlJpkMagGenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsPlJpkMagGenerateResponseBuilder::source)
    /// - [`warehouse_code`](PostV1DeclarationsPlJpkMagGenerateResponseBuilder::warehouse_code)
    /// - [`counts`](PostV1DeclarationsPlJpkMagGenerateResponseBuilder::counts)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkMagGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsPlJpkMagGenerateResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            warehouse_code: self
                .warehouse_code
                .ok_or_else(|| BuildError::missing_field("warehouse_code"))?,
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
        })
    }
}
