pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtMunicipalitiesListReferenceResponse {
    #[serde(default)]
    pub rows: Vec<LtMunicipalitiesListReferenceResponseRowsItem>,
}

impl LtMunicipalitiesListReferenceResponse {
    pub fn builder() -> LtMunicipalitiesListReferenceResponseBuilder {
        <LtMunicipalitiesListReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtMunicipalitiesListReferenceResponseBuilder {
    rows: Option<Vec<LtMunicipalitiesListReferenceResponseRowsItem>>,
}

impl LtMunicipalitiesListReferenceResponseBuilder {
    pub fn rows(mut self, value: Vec<LtMunicipalitiesListReferenceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtMunicipalitiesListReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](LtMunicipalitiesListReferenceResponseBuilder::rows)
    pub fn build(self) -> Result<LtMunicipalitiesListReferenceResponse, BuildError> {
        Ok(LtMunicipalitiesListReferenceResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
