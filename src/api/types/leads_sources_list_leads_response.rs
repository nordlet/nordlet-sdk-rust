pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourcesListLeadsResponse {
    #[serde(default)]
    pub rows: Vec<SourcesListLeadsResponseRowsItem>,
}

impl SourcesListLeadsResponse {
    pub fn builder() -> SourcesListLeadsResponseBuilder {
        <SourcesListLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourcesListLeadsResponseBuilder {
    rows: Option<Vec<SourcesListLeadsResponseRowsItem>>,
}

impl SourcesListLeadsResponseBuilder {
    pub fn rows(mut self, value: Vec<SourcesListLeadsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourcesListLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](SourcesListLeadsResponseBuilder::rows)
    pub fn build(self) -> Result<SourcesListLeadsResponse, BuildError> {
        Ok(SourcesListLeadsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
