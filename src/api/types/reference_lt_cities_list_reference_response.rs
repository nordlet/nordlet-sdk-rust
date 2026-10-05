pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtCitiesListReferenceResponse {
    #[serde(default)]
    pub rows: Vec<LtCitiesListReferenceResponseRowsItem>,
}

impl LtCitiesListReferenceResponse {
    pub fn builder() -> LtCitiesListReferenceResponseBuilder {
        <LtCitiesListReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtCitiesListReferenceResponseBuilder {
    rows: Option<Vec<LtCitiesListReferenceResponseRowsItem>>,
}

impl LtCitiesListReferenceResponseBuilder {
    pub fn rows(mut self, value: Vec<LtCitiesListReferenceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtCitiesListReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](LtCitiesListReferenceResponseBuilder::rows)
    pub fn build(self) -> Result<LtCitiesListReferenceResponse, BuildError> {
        Ok(LtCitiesListReferenceResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
