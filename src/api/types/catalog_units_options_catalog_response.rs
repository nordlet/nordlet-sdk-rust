pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsOptionsCatalogResponse {
    #[serde(default)]
    pub rows: Vec<UnitsOptionsCatalogResponseRowsItem>,
}

impl UnitsOptionsCatalogResponse {
    pub fn builder() -> UnitsOptionsCatalogResponseBuilder {
        <UnitsOptionsCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsOptionsCatalogResponseBuilder {
    rows: Option<Vec<UnitsOptionsCatalogResponseRowsItem>>,
}

impl UnitsOptionsCatalogResponseBuilder {
    pub fn rows(mut self, value: Vec<UnitsOptionsCatalogResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitsOptionsCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](UnitsOptionsCatalogResponseBuilder::rows)
    pub fn build(self) -> Result<UnitsOptionsCatalogResponse, BuildError> {
        Ok(UnitsOptionsCatalogResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
