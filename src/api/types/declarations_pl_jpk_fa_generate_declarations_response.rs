pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkFaGenerateDeclarationsResponse {
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
    #[serde(default)]
    pub counts: PlJpkFaGenerateDeclarationsResponseCounts,
    #[serde(default)]
    pub totals: PlJpkFaGenerateDeclarationsResponseTotals,
}

impl PlJpkFaGenerateDeclarationsResponse {
    pub fn builder() -> PlJpkFaGenerateDeclarationsResponseBuilder {
        <PlJpkFaGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkFaGenerateDeclarationsResponseBuilder {
    file_name: Option<String>,
    xml: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
    counts: Option<PlJpkFaGenerateDeclarationsResponseCounts>,
    totals: Option<PlJpkFaGenerateDeclarationsResponseTotals>,
}

impl PlJpkFaGenerateDeclarationsResponseBuilder {
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

    pub fn counts(mut self, value: PlJpkFaGenerateDeclarationsResponseCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn totals(mut self, value: PlJpkFaGenerateDeclarationsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlJpkFaGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PlJpkFaGenerateDeclarationsResponseBuilder::file_name)
    /// - [`xml`](PlJpkFaGenerateDeclarationsResponseBuilder::xml)
    /// - [`period_start`](PlJpkFaGenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](PlJpkFaGenerateDeclarationsResponseBuilder::period_end)
    /// - [`warnings`](PlJpkFaGenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](PlJpkFaGenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](PlJpkFaGenerateDeclarationsResponseBuilder::source)
    /// - [`counts`](PlJpkFaGenerateDeclarationsResponseBuilder::counts)
    /// - [`totals`](PlJpkFaGenerateDeclarationsResponseBuilder::totals)
    pub fn build(self) -> Result<PlJpkFaGenerateDeclarationsResponse, BuildError> {
        Ok(PlJpkFaGenerateDeclarationsResponse {
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
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
