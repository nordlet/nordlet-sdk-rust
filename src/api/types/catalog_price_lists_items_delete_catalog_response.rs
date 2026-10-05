pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsItemsDeleteCatalogResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PriceListsItemsDeleteCatalogResponse {
    pub fn builder() -> PriceListsItemsDeleteCatalogResponseBuilder {
        <PriceListsItemsDeleteCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsItemsDeleteCatalogResponseBuilder {
    deleted: Option<bool>,
}

impl PriceListsItemsDeleteCatalogResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceListsItemsDeleteCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PriceListsItemsDeleteCatalogResponseBuilder::deleted)
    pub fn build(self) -> Result<PriceListsItemsDeleteCatalogResponse, BuildError> {
        Ok(PriceListsItemsDeleteCatalogResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
