pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CyTd4GenerateDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(rename = "taxIdentificationCode")]
    #[serde(default)]
    pub tax_identification_code: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub fields: Vec<CyTd4GenerateDeclarationsResponseFieldsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl CyTd4GenerateDeclarationsResponse {
    pub fn builder() -> CyTd4GenerateDeclarationsResponseBuilder {
        <CyTd4GenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CyTd4GenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    tax_identification_code: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    fields: Option<Vec<CyTd4GenerateDeclarationsResponseFieldsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl CyTd4GenerateDeclarationsResponseBuilder {
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

    pub fn tax_identification_code(mut self, value: impl Into<String>) -> Self {
        self.tax_identification_code = Some(value.into());
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

    pub fn fields(mut self, value: Vec<CyTd4GenerateDeclarationsResponseFieldsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`CyTd4GenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](CyTd4GenerateDeclarationsResponseBuilder::year)
    /// - [`period_start`](CyTd4GenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](CyTd4GenerateDeclarationsResponseBuilder::period_end)
    /// - [`tax_identification_code`](CyTd4GenerateDeclarationsResponseBuilder::tax_identification_code)
    /// - [`file_name`](CyTd4GenerateDeclarationsResponseBuilder::file_name)
    /// - [`xml`](CyTd4GenerateDeclarationsResponseBuilder::xml)
    /// - [`fields`](CyTd4GenerateDeclarationsResponseBuilder::fields)
    /// - [`warnings`](CyTd4GenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](CyTd4GenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](CyTd4GenerateDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<CyTd4GenerateDeclarationsResponse, BuildError> {
        Ok(CyTd4GenerateDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            tax_identification_code: self
                .tax_identification_code
                .ok_or_else(|| BuildError::missing_field("tax_identification_code"))?,
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
