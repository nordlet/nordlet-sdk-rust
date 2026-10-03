pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLiLohnlistenGenerateResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsLiLohnlistenGenerateResponseRowsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsLiLohnlistenGenerateResponse {
    pub fn builder() -> PostV1DeclarationsLiLohnlistenGenerateResponseBuilder {
        <PostV1DeclarationsLiLohnlistenGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLiLohnlistenGenerateResponseBuilder {
    year: Option<i64>,
    file_name: Option<String>,
    content: Option<String>,
    rows: Option<Vec<PostV1DeclarationsLiLohnlistenGenerateResponseRowsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsLiLohnlistenGenerateResponseBuilder {
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
        value: Vec<PostV1DeclarationsLiLohnlistenGenerateResponseRowsItem>,
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLiLohnlistenGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLiLohnlistenGenerateResponseBuilder::year)
    /// - [`file_name`](PostV1DeclarationsLiLohnlistenGenerateResponseBuilder::file_name)
    /// - [`content`](PostV1DeclarationsLiLohnlistenGenerateResponseBuilder::content)
    /// - [`rows`](PostV1DeclarationsLiLohnlistenGenerateResponseBuilder::rows)
    /// - [`warnings`](PostV1DeclarationsLiLohnlistenGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLiLohnlistenGenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsLiLohnlistenGenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsLiLohnlistenGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsLiLohnlistenGenerateResponse {
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
