pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkV7MGenerateResponse {
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
    pub declaration: Vec<PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItem>,
    #[serde(default)]
    pub counts: PostV1DeclarationsPlJpkV7MGenerateResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1DeclarationsPlJpkV7MGenerateResponse {
    pub fn builder() -> PostV1DeclarationsPlJpkV7MGenerateResponseBuilder {
        <PostV1DeclarationsPlJpkV7MGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkV7MGenerateResponseBuilder {
    file_name: Option<String>,
    xml: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    declaration: Option<Vec<PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItem>>,
    counts: Option<PostV1DeclarationsPlJpkV7MGenerateResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PostV1DeclarationsPlJpkV7MGenerateResponseBuilder {
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

    pub fn declaration(
        mut self,
        value: Vec<PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItem>,
    ) -> Self {
        self.declaration = Some(value);
        self
    }

    pub fn counts(mut self, value: PostV1DeclarationsPlJpkV7MGenerateResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkV7MGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PostV1DeclarationsPlJpkV7MGenerateResponseBuilder::file_name)
    /// - [`xml`](PostV1DeclarationsPlJpkV7MGenerateResponseBuilder::xml)
    /// - [`period_start`](PostV1DeclarationsPlJpkV7MGenerateResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsPlJpkV7MGenerateResponseBuilder::period_end)
    /// - [`declaration`](PostV1DeclarationsPlJpkV7MGenerateResponseBuilder::declaration)
    /// - [`counts`](PostV1DeclarationsPlJpkV7MGenerateResponseBuilder::counts)
    /// - [`warnings`](PostV1DeclarationsPlJpkV7MGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsPlJpkV7MGenerateResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkV7MGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsPlJpkV7MGenerateResponse {
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
            declaration: self
                .declaration
                .ok_or_else(|| BuildError::missing_field("declaration"))?,
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
