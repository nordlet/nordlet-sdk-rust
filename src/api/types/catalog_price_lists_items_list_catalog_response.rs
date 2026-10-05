pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsItemsListCatalogResponse {
    #[serde(default)]
    pub rows: Vec<PriceListsItemsListCatalogResponseRowsItem>,
}

impl PriceListsItemsListCatalogResponse {
    pub fn builder() -> PriceListsItemsListCatalogResponseBuilder {
        <PriceListsItemsListCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsItemsListCatalogResponseBuilder {
    rows: Option<Vec<PriceListsItemsListCatalogResponseRowsItem>>,
}

impl PriceListsItemsListCatalogResponseBuilder {
    pub fn rows(mut self, value: Vec<PriceListsItemsListCatalogResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceListsItemsListCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PriceListsItemsListCatalogResponseBuilder::rows)
    pub fn build(self) -> Result<PriceListsItemsListCatalogResponse, BuildError> {
        Ok(PriceListsItemsListCatalogResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
