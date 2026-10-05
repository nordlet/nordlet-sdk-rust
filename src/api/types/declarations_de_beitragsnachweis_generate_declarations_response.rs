pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeBeitragsnachweisGenerateDeclarationsResponse {
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
    pub records: Vec<DeBeitragsnachweisGenerateDeclarationsResponseRecordsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl DeBeitragsnachweisGenerateDeclarationsResponse {
    pub fn builder() -> DeBeitragsnachweisGenerateDeclarationsResponseBuilder {
        <DeBeitragsnachweisGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeBeitragsnachweisGenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    month: Option<i64>,
    file_name: Option<String>,
    content: Option<String>,
    source: Option<String>,
    records: Option<Vec<DeBeitragsnachweisGenerateDeclarationsResponseRecordsItem>>,
    warnings: Option<Vec<String>>,
}

impl DeBeitragsnachweisGenerateDeclarationsResponseBuilder {
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
        value: Vec<DeBeitragsnachweisGenerateDeclarationsResponseRecordsItem>,
    ) -> Self {
        self.records = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeBeitragsnachweisGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeBeitragsnachweisGenerateDeclarationsResponseBuilder::year)
    /// - [`month`](DeBeitragsnachweisGenerateDeclarationsResponseBuilder::month)
    /// - [`file_name`](DeBeitragsnachweisGenerateDeclarationsResponseBuilder::file_name)
    /// - [`content`](DeBeitragsnachweisGenerateDeclarationsResponseBuilder::content)
    /// - [`source`](DeBeitragsnachweisGenerateDeclarationsResponseBuilder::source)
    /// - [`records`](DeBeitragsnachweisGenerateDeclarationsResponseBuilder::records)
    /// - [`warnings`](DeBeitragsnachweisGenerateDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<DeBeitragsnachweisGenerateDeclarationsResponse, BuildError> {
        Ok(DeBeitragsnachweisGenerateDeclarationsResponse {
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
        })
    }
}
