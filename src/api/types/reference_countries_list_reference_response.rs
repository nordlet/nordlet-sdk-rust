pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CountriesListReferenceResponse {
    #[serde(default)]
    pub rows: Vec<CountriesListReferenceResponseRowsItem>,
}

impl CountriesListReferenceResponse {
    pub fn builder() -> CountriesListReferenceResponseBuilder {
        <CountriesListReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CountriesListReferenceResponseBuilder {
    rows: Option<Vec<CountriesListReferenceResponseRowsItem>>,
}

impl CountriesListReferenceResponseBuilder {
    pub fn rows(mut self, value: Vec<CountriesListReferenceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CountriesListReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](CountriesListReferenceResponseBuilder::rows)
    pub fn build(self) -> Result<CountriesListReferenceResponse, BuildError> {
        Ok(CountriesListReferenceResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
