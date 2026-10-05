pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DkSelskabsskatGenerateDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(rename = "cvrNummer")]
    #[serde(default)]
    pub cvr_nummer: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub fields: Vec<DkSelskabsskatGenerateDeclarationsResponseFieldsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl DkSelskabsskatGenerateDeclarationsResponse {
    pub fn builder() -> DkSelskabsskatGenerateDeclarationsResponseBuilder {
        <DkSelskabsskatGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DkSelskabsskatGenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    cvr_nummer: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    fields: Option<Vec<DkSelskabsskatGenerateDeclarationsResponseFieldsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl DkSelskabsskatGenerateDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
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

    pub fn cvr_nummer(mut self, value: impl Into<String>) -> Self {
        self.cvr_nummer = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    pub fn fields(
        mut self,
        value: Vec<DkSelskabsskatGenerateDeclarationsResponseFieldsItem>,
    ) -> Self {
        self.fields = Some(value);
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

    /// Consumes the builder and constructs a [`DkSelskabsskatGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DkSelskabsskatGenerateDeclarationsResponseBuilder::year)
    /// - [`period_start`](DkSelskabsskatGenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](DkSelskabsskatGenerateDeclarationsResponseBuilder::period_end)
    /// - [`cvr_nummer`](DkSelskabsskatGenerateDeclarationsResponseBuilder::cvr_nummer)
    /// - [`file_name`](DkSelskabsskatGenerateDeclarationsResponseBuilder::file_name)
    /// - [`xml`](DkSelskabsskatGenerateDeclarationsResponseBuilder::xml)
    /// - [`fields`](DkSelskabsskatGenerateDeclarationsResponseBuilder::fields)
    /// - [`warnings`](DkSelskabsskatGenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](DkSelskabsskatGenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](DkSelskabsskatGenerateDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<DkSelskabsskatGenerateDeclarationsResponse, BuildError> {
        Ok(DkSelskabsskatGenerateDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            cvr_nummer: self
                .cvr_nummer
                .ok_or_else(|| BuildError::missing_field("cvr_nummer"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
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
