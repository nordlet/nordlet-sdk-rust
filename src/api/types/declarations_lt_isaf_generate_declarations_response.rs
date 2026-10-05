pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIsafGenerateDeclarationsResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub counts: LtIsafGenerateDeclarationsResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub xml: String,
}

impl LtIsafGenerateDeclarationsResponse {
    pub fn builder() -> LtIsafGenerateDeclarationsResponseBuilder {
        <LtIsafGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIsafGenerateDeclarationsResponseBuilder {
    file_name: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    counts: Option<LtIsafGenerateDeclarationsResponseCounts>,
    warnings: Option<Vec<String>>,
    xml: Option<String>,
}

impl LtIsafGenerateDeclarationsResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
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

    pub fn counts(mut self, value: LtIsafGenerateDeclarationsResponseCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtIsafGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](LtIsafGenerateDeclarationsResponseBuilder::file_name)
    /// - [`period_start`](LtIsafGenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](LtIsafGenerateDeclarationsResponseBuilder::period_end)
    /// - [`counts`](LtIsafGenerateDeclarationsResponseBuilder::counts)
    /// - [`warnings`](LtIsafGenerateDeclarationsResponseBuilder::warnings)
    /// - [`xml`](LtIsafGenerateDeclarationsResponseBuilder::xml)
    pub fn build(self) -> Result<LtIsafGenerateDeclarationsResponse, BuildError> {
        Ok(LtIsafGenerateDeclarationsResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
        })
    }
}
