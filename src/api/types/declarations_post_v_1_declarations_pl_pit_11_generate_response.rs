pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlPit11GenerateResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub persons: Vec<PostV1DeclarationsPlPit11GenerateResponsePersonsItem>,
}

impl PostV1DeclarationsPlPit11GenerateResponse {
    pub fn builder() -> PostV1DeclarationsPlPit11GenerateResponseBuilder {
        <PostV1DeclarationsPlPit11GenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlPit11GenerateResponseBuilder {
    year: Option<i64>,
    source: Option<String>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    persons: Option<Vec<PostV1DeclarationsPlPit11GenerateResponsePersonsItem>>,
}

impl PostV1DeclarationsPlPit11GenerateResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
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

    pub fn persons(
        mut self,
        value: Vec<PostV1DeclarationsPlPit11GenerateResponsePersonsItem>,
    ) -> Self {
        self.persons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlPit11GenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsPlPit11GenerateResponseBuilder::year)
    /// - [`source`](PostV1DeclarationsPlPit11GenerateResponseBuilder::source)
    /// - [`warnings`](PostV1DeclarationsPlPit11GenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsPlPit11GenerateResponseBuilder::notes)
    /// - [`persons`](PostV1DeclarationsPlPit11GenerateResponseBuilder::persons)
    pub fn build(self) -> Result<PostV1DeclarationsPlPit11GenerateResponse, BuildError> {
        Ok(PostV1DeclarationsPlPit11GenerateResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            persons: self
                .persons
                .ok_or_else(|| BuildError::missing_field("persons"))?,
        })
    }
}
