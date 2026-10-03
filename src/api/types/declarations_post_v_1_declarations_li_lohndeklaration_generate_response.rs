pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLiLohndeklarationGenerateResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsLiLohndeklarationGenerateResponse {
    pub fn builder() -> PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder {
        <PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder {
    year: Option<i64>,
    file_name: Option<String>,
    content: Option<String>,
    rows: Option<Vec<PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn rows(
        mut self,
        value: Vec<PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLiLohndeklarationGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder::year)
    /// - [`file_name`](PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder::file_name)
    /// - [`content`](PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder::content)
    /// - [`rows`](PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder::rows)
    /// - [`warnings`](PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsLiLohndeklarationGenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsLiLohndeklarationGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsLiLohndeklarationGenerateResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
