pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlPit11GenerateDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub persons: Vec<PlPit11GenerateDeclarationsResponsePersonsItem>,
}

impl PlPit11GenerateDeclarationsResponse {
    pub fn builder() -> PlPit11GenerateDeclarationsResponseBuilder {
        <PlPit11GenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlPit11GenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    source: Option<String>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    persons: Option<Vec<PlPit11GenerateDeclarationsResponsePersonsItem>>,
}

impl PlPit11GenerateDeclarationsResponseBuilder {
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

    pub fn persons(mut self, value: Vec<PlPit11GenerateDeclarationsResponsePersonsItem>) -> Self {
        self.persons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlPit11GenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PlPit11GenerateDeclarationsResponseBuilder::year)
    /// - [`source`](PlPit11GenerateDeclarationsResponseBuilder::source)
    /// - [`warnings`](PlPit11GenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](PlPit11GenerateDeclarationsResponseBuilder::notes)
    /// - [`persons`](PlPit11GenerateDeclarationsResponseBuilder::persons)
    pub fn build(self) -> Result<PlPit11GenerateDeclarationsResponse, BuildError> {
        Ok(PlPit11GenerateDeclarationsResponse {
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
