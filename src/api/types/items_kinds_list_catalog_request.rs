pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsKindsListCatalogRequest {}

impl ItemsKindsListCatalogRequest {
    pub fn builder() -> ItemsKindsListCatalogRequestBuilder {
        <ItemsKindsListCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsKindsListCatalogRequestBuilder {}

impl ItemsKindsListCatalogRequestBuilder {
    /// Consumes the builder and constructs a [`ItemsKindsListCatalogRequest`].
    pub fn build(self) -> Result<ItemsKindsListCatalogRequest, BuildError> {
        Ok(ItemsKindsListCatalogRequest {})
    }
}
