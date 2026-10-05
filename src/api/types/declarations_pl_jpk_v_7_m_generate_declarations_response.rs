pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkV7MGenerateDeclarationsResponse {
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
    pub declaration: Vec<PlJpkV7MGenerateDeclarationsResponseDeclarationItem>,
    #[serde(default)]
    pub counts: PlJpkV7MGenerateDeclarationsResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PlJpkV7MGenerateDeclarationsResponse {
    pub fn builder() -> PlJpkV7MGenerateDeclarationsResponseBuilder {
        <PlJpkV7MGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkV7MGenerateDeclarationsResponseBuilder {
    file_name: Option<String>,
    xml: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    declaration: Option<Vec<PlJpkV7MGenerateDeclarationsResponseDeclarationItem>>,
    counts: Option<PlJpkV7MGenerateDeclarationsResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PlJpkV7MGenerateDeclarationsResponseBuilder {
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

    pub fn declaration(
        mut self,
        value: Vec<PlJpkV7MGenerateDeclarationsResponseDeclarationItem>,
    ) -> Self {
        self.declaration = Some(value);
        self
    }

    pub fn counts(mut self, value: PlJpkV7MGenerateDeclarationsResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`PlJpkV7MGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PlJpkV7MGenerateDeclarationsResponseBuilder::file_name)
    /// - [`xml`](PlJpkV7MGenerateDeclarationsResponseBuilder::xml)
    /// - [`period_start`](PlJpkV7MGenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](PlJpkV7MGenerateDeclarationsResponseBuilder::period_end)
    /// - [`declaration`](PlJpkV7MGenerateDeclarationsResponseBuilder::declaration)
    /// - [`counts`](PlJpkV7MGenerateDeclarationsResponseBuilder::counts)
    /// - [`warnings`](PlJpkV7MGenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](PlJpkV7MGenerateDeclarationsResponseBuilder::notes)
    pub fn build(self) -> Result<PlJpkV7MGenerateDeclarationsResponse, BuildError> {
        Ok(PlJpkV7MGenerateDeclarationsResponse {
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
            declaration: self
                .declaration
                .ok_or_else(|| BuildError::missing_field("declaration"))?,
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
