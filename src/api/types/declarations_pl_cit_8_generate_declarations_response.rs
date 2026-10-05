pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlCit8GenerateDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub positions: Vec<PlCit8GenerateDeclarationsResponsePositionsItem>,
    #[serde(default)]
    pub annexes: Vec<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PlCit8GenerateDeclarationsResponse {
    pub fn builder() -> PlCit8GenerateDeclarationsResponseBuilder {
        <PlCit8GenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlCit8GenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    positions: Option<Vec<PlCit8GenerateDeclarationsResponsePositionsItem>>,
    annexes: Option<Vec<String>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PlCit8GenerateDeclarationsResponseBuilder {
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

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    pub fn positions(
        mut self,
        value: Vec<PlCit8GenerateDeclarationsResponsePositionsItem>,
    ) -> Self {
        self.positions = Some(value);
        self
    }

    pub fn annexes(mut self, value: Vec<String>) -> Self {
        self.annexes = Some(value);
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

    /// Consumes the builder and constructs a [`PlCit8GenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PlCit8GenerateDeclarationsResponseBuilder::year)
    /// - [`period_start`](PlCit8GenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](PlCit8GenerateDeclarationsResponseBuilder::period_end)
    /// - [`file_name`](PlCit8GenerateDeclarationsResponseBuilder::file_name)
    /// - [`xml`](PlCit8GenerateDeclarationsResponseBuilder::xml)
    /// - [`positions`](PlCit8GenerateDeclarationsResponseBuilder::positions)
    /// - [`annexes`](PlCit8GenerateDeclarationsResponseBuilder::annexes)
    /// - [`warnings`](PlCit8GenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](PlCit8GenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](PlCit8GenerateDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<PlCit8GenerateDeclarationsResponse, BuildError> {
        Ok(PlCit8GenerateDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            positions: self
                .positions
                .ok_or_else(|| BuildError::missing_field("positions"))?,
            annexes: self
                .annexes
                .ok_or_else(|| BuildError::missing_field("annexes"))?,
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
