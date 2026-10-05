pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FecReportsResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub rows: i64,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl FecReportsResponse {
    pub fn builder() -> FecReportsResponseBuilder {
        <FecReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FecReportsResponseBuilder {
    file_name: Option<String>,
    content_type: Option<String>,
    data: Option<String>,
    rows: Option<i64>,
    source: Option<String>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl FecReportsResponseBuilder {
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

    pub fn rows(mut self, value: i64) -> Self {
        self.rows = Some(value);
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

    /// Consumes the builder and constructs a [`FecReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](FecReportsResponseBuilder::file_name)
    /// - [`content_type`](FecReportsResponseBuilder::content_type)
    /// - [`data`](FecReportsResponseBuilder::data)
    /// - [`rows`](FecReportsResponseBuilder::rows)
    /// - [`source`](FecReportsResponseBuilder::source)
    /// - [`warnings`](FecReportsResponseBuilder::warnings)
    /// - [`notes`](FecReportsResponseBuilder::notes)
    pub fn build(self) -> Result<FecReportsResponse, BuildError> {
        Ok(FecReportsResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
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
