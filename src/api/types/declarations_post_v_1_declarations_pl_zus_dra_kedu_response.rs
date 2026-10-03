pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlZusDraKeduResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub insured: Vec<PostV1DeclarationsPlZusDraKeduResponseInsuredItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1DeclarationsPlZusDraKeduResponse {
    pub fn builder() -> PostV1DeclarationsPlZusDraKeduResponseBuilder {
        <PostV1DeclarationsPlZusDraKeduResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlZusDraKeduResponseBuilder {
    year: Option<i64>,
    month: Option<i64>,
    file_name: Option<String>,
    xml: Option<String>,
    source: Option<String>,
    insured: Option<Vec<PostV1DeclarationsPlZusDraKeduResponseInsuredItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PostV1DeclarationsPlZusDraKeduResponseBuilder {
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

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn insured(
        mut self,
        value: Vec<PostV1DeclarationsPlZusDraKeduResponseInsuredItem>,
    ) -> Self {
        self.insured = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlZusDraKeduResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsPlZusDraKeduResponseBuilder::year)
    /// - [`month`](PostV1DeclarationsPlZusDraKeduResponseBuilder::month)
    /// - [`file_name`](PostV1DeclarationsPlZusDraKeduResponseBuilder::file_name)
    /// - [`xml`](PostV1DeclarationsPlZusDraKeduResponseBuilder::xml)
    /// - [`source`](PostV1DeclarationsPlZusDraKeduResponseBuilder::source)
    /// - [`insured`](PostV1DeclarationsPlZusDraKeduResponseBuilder::insured)
    /// - [`warnings`](PostV1DeclarationsPlZusDraKeduResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsPlZusDraKeduResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1DeclarationsPlZusDraKeduResponse, BuildError> {
        Ok(PostV1DeclarationsPlZusDraKeduResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            insured: self
                .insured
                .ok_or_else(|| BuildError::missing_field("insured"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
