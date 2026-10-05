pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourcesOptionsLeadsResponse {
    #[serde(default)]
    pub rows: Vec<SourcesOptionsLeadsResponseRowsItem>,
}

impl SourcesOptionsLeadsResponse {
    pub fn builder() -> SourcesOptionsLeadsResponseBuilder {
        <SourcesOptionsLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourcesOptionsLeadsResponseBuilder {
    rows: Option<Vec<SourcesOptionsLeadsResponseRowsItem>>,
}

impl SourcesOptionsLeadsResponseBuilder {
    pub fn rows(mut self, value: Vec<SourcesOptionsLeadsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourcesOptionsLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](SourcesOptionsLeadsResponseBuilder::rows)
    pub fn build(self) -> Result<SourcesOptionsLeadsResponse, BuildError> {
        Ok(SourcesOptionsLeadsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
