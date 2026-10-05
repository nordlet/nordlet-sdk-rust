pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeDeuevGenerateDeclarationsResponse {
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
    pub records: Vec<DeDeuevGenerateDeclarationsResponseRecordsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl DeDeuevGenerateDeclarationsResponse {
    pub fn builder() -> DeDeuevGenerateDeclarationsResponseBuilder {
        <DeDeuevGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeDeuevGenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    month: Option<i64>,
    file_name: Option<String>,
    content: Option<String>,
    source: Option<String>,
    records: Option<Vec<DeDeuevGenerateDeclarationsResponseRecordsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl DeDeuevGenerateDeclarationsResponseBuilder {
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

    pub fn records(mut self, value: Vec<DeDeuevGenerateDeclarationsResponseRecordsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`DeDeuevGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeDeuevGenerateDeclarationsResponseBuilder::year)
    /// - [`month`](DeDeuevGenerateDeclarationsResponseBuilder::month)
    /// - [`file_name`](DeDeuevGenerateDeclarationsResponseBuilder::file_name)
    /// - [`content`](DeDeuevGenerateDeclarationsResponseBuilder::content)
    /// - [`source`](DeDeuevGenerateDeclarationsResponseBuilder::source)
    /// - [`records`](DeDeuevGenerateDeclarationsResponseBuilder::records)
    /// - [`warnings`](DeDeuevGenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](DeDeuevGenerateDeclarationsResponseBuilder::notes)
    pub fn build(self) -> Result<DeDeuevGenerateDeclarationsResponse, BuildError> {
        Ok(DeDeuevGenerateDeclarationsResponse {
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
