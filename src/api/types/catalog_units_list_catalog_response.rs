pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsListCatalogResponse {
    #[serde(default)]
    pub rows: Vec<UnitsListCatalogResponseRowsItem>,
}

impl UnitsListCatalogResponse {
    pub fn builder() -> UnitsListCatalogResponseBuilder {
        <UnitsListCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsListCatalogResponseBuilder {
    rows: Option<Vec<UnitsListCatalogResponseRowsItem>>,
}

impl UnitsListCatalogResponseBuilder {
    pub fn rows(mut self, value: Vec<UnitsListCatalogResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitsListCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](UnitsListCatalogResponseBuilder::rows)
    pub fn build(self) -> Result<UnitsListCatalogResponse, BuildError> {
        Ok(UnitsListCatalogResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
