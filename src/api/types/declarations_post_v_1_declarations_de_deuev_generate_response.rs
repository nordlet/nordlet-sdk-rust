pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeDeuevGenerateResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub records: Vec<PostV1DeclarationsDeDeuevGenerateResponseRecordsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1DeclarationsDeDeuevGenerateResponse {
    pub fn builder() -> PostV1DeclarationsDeDeuevGenerateResponseBuilder {
        <PostV1DeclarationsDeDeuevGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeDeuevGenerateResponseBuilder {
    year: Option<i64>,
    month: Option<i64>,
    file_name: Option<String>,
    content: Option<String>,
    source: Option<String>,
    records: Option<Vec<PostV1DeclarationsDeDeuevGenerateResponseRecordsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PostV1DeclarationsDeDeuevGenerateResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
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

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn records(
        mut self,
        value: Vec<PostV1DeclarationsDeDeuevGenerateResponseRecordsItem>,
    ) -> Self {
        self.records = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeDeuevGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsDeDeuevGenerateResponseBuilder::year)
    /// - [`month`](PostV1DeclarationsDeDeuevGenerateResponseBuilder::month)
    /// - [`file_name`](PostV1DeclarationsDeDeuevGenerateResponseBuilder::file_name)
    /// - [`content`](PostV1DeclarationsDeDeuevGenerateResponseBuilder::content)
    /// - [`source`](PostV1DeclarationsDeDeuevGenerateResponseBuilder::source)
    /// - [`records`](PostV1DeclarationsDeDeuevGenerateResponseBuilder::records)
    /// - [`warnings`](PostV1DeclarationsDeDeuevGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsDeDeuevGenerateResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1DeclarationsDeDeuevGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsDeDeuevGenerateResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
