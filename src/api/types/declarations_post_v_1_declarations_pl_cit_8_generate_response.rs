pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlCit8GenerateResponse {
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
    pub positions: Vec<PostV1DeclarationsPlCit8GenerateResponsePositionsItem>,
    #[serde(default)]
    pub annexes: Vec<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsPlCit8GenerateResponse {
    pub fn builder() -> PostV1DeclarationsPlCit8GenerateResponseBuilder {
        <PostV1DeclarationsPlCit8GenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlCit8GenerateResponseBuilder {
    year: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    positions: Option<Vec<PostV1DeclarationsPlCit8GenerateResponsePositionsItem>>,
    annexes: Option<Vec<String>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsPlCit8GenerateResponseBuilder {
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
        value: Vec<PostV1DeclarationsPlCit8GenerateResponsePositionsItem>,
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlCit8GenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsPlCit8GenerateResponseBuilder::year)
    /// - [`period_start`](PostV1DeclarationsPlCit8GenerateResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsPlCit8GenerateResponseBuilder::period_end)
    /// - [`file_name`](PostV1DeclarationsPlCit8GenerateResponseBuilder::file_name)
    /// - [`xml`](PostV1DeclarationsPlCit8GenerateResponseBuilder::xml)
    /// - [`positions`](PostV1DeclarationsPlCit8GenerateResponseBuilder::positions)
    /// - [`annexes`](PostV1DeclarationsPlCit8GenerateResponseBuilder::annexes)
    /// - [`warnings`](PostV1DeclarationsPlCit8GenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsPlCit8GenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsPlCit8GenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsPlCit8GenerateResponse, BuildError> {
        Ok(PostV1DeclarationsPlCit8GenerateResponse {
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
