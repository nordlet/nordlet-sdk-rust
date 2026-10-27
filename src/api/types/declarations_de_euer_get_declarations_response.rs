pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeEuerGetDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub rows: Vec<DeEuerGetDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl DeEuerGetDeclarationsResponse {
    pub fn builder() -> DeEuerGetDeclarationsResponseBuilder {
        <DeEuerGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeEuerGetDeclarationsResponseBuilder {
    year: Option<i64>,
    rows: Option<Vec<DeEuerGetDeclarationsResponseRowsItem>>,
    warnings: Option<Vec<String>>,
    source: Option<String>,
}

impl DeEuerGetDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<DeEuerGetDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeEuerGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeEuerGetDeclarationsResponseBuilder::year)
    /// - [`rows`](DeEuerGetDeclarationsResponseBuilder::rows)
    /// - [`warnings`](DeEuerGetDeclarationsResponseBuilder::warnings)
    /// - [`source`](DeEuerGetDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<DeEuerGetDeclarationsResponse, BuildError> {
        Ok(DeEuerGetDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
