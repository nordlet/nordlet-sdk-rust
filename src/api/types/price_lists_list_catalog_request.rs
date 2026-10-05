pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsListCatalogRequest {}

impl PriceListsListCatalogRequest {
    pub fn builder() -> PriceListsListCatalogRequestBuilder {
        <PriceListsListCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsListCatalogRequestBuilder {}

impl PriceListsListCatalogRequestBuilder {
    /// Consumes the builder and constructs a [`PriceListsListCatalogRequest`].
    pub fn build(self) -> Result<PriceListsListCatalogRequest, BuildError> {
        Ok(PriceListsListCatalogRequest {})
    }
}
