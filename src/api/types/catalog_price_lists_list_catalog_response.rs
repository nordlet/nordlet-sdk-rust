pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsListCatalogResponse {
    #[serde(default)]
    pub rows: Vec<PriceListsListCatalogResponseRowsItem>,
}

impl PriceListsListCatalogResponse {
    pub fn builder() -> PriceListsListCatalogResponseBuilder {
        <PriceListsListCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsListCatalogResponseBuilder {
    rows: Option<Vec<PriceListsListCatalogResponseRowsItem>>,
}

impl PriceListsListCatalogResponseBuilder {
    pub fn rows(mut self, value: Vec<PriceListsListCatalogResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceListsListCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PriceListsListCatalogResponseBuilder::rows)
    pub fn build(self) -> Result<PriceListsListCatalogResponse, BuildError> {
        Ok(PriceListsListCatalogResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
