pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIvazAmendDeclarationsResponse {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default)]
    pub counts: LtIvazAmendDeclarationsResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub xml: String,
}

impl LtIvazAmendDeclarationsResponse {
    pub fn builder() -> LtIvazAmendDeclarationsResponseBuilder {
        <LtIvazAmendDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIvazAmendDeclarationsResponseBuilder {
    file_name: Option<String>,
    file_id: Option<String>,
    counts: Option<LtIvazAmendDeclarationsResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    xml: Option<String>,
}

impl LtIvazAmendDeclarationsResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn counts(mut self, value: LtIvazAmendDeclarationsResponseCounts) -> Self {
        self.counts = Some(value);
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

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtIvazAmendDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](LtIvazAmendDeclarationsResponseBuilder::file_name)
    /// - [`counts`](LtIvazAmendDeclarationsResponseBuilder::counts)
    /// - [`warnings`](LtIvazAmendDeclarationsResponseBuilder::warnings)
    /// - [`notes`](LtIvazAmendDeclarationsResponseBuilder::notes)
    /// - [`xml`](LtIvazAmendDeclarationsResponseBuilder::xml)
    pub fn build(self) -> Result<LtIvazAmendDeclarationsResponse, BuildError> {
        Ok(LtIvazAmendDeclarationsResponse {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file_id: self.file_id,
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
        })
    }
}
