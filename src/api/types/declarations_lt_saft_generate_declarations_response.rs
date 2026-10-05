pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtSaftGenerateDeclarationsResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub counts: LtSaftGenerateDeclarationsResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub xml: String,
}

impl LtSaftGenerateDeclarationsResponse {
    pub fn builder() -> LtSaftGenerateDeclarationsResponseBuilder {
        <LtSaftGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtSaftGenerateDeclarationsResponseBuilder {
    file_name: Option<String>,
    file_id: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    counts: Option<LtSaftGenerateDeclarationsResponseCounts>,
    warnings: Option<Vec<String>>,
    xml: Option<String>,
}

impl LtSaftGenerateDeclarationsResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
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

    pub fn counts(mut self, value: LtSaftGenerateDeclarationsResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`LtSaftGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](LtSaftGenerateDeclarationsResponseBuilder::file_name)
    /// - [`period_start`](LtSaftGenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](LtSaftGenerateDeclarationsResponseBuilder::period_end)
    /// - [`counts`](LtSaftGenerateDeclarationsResponseBuilder::counts)
    /// - [`warnings`](LtSaftGenerateDeclarationsResponseBuilder::warnings)
    /// - [`xml`](LtSaftGenerateDeclarationsResponseBuilder::xml)
    pub fn build(self) -> Result<LtSaftGenerateDeclarationsResponse, BuildError> {
        Ok(LtSaftGenerateDeclarationsResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file_id: self.file_id,
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
