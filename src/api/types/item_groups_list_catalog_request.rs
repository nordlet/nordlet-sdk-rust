pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemGroupsListCatalogRequest {}

impl ItemGroupsListCatalogRequest {
    pub fn builder() -> ItemGroupsListCatalogRequestBuilder {
        <ItemGroupsListCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemGroupsListCatalogRequestBuilder {}

impl ItemGroupsListCatalogRequestBuilder {
    /// Consumes the builder and constructs a [`ItemGroupsListCatalogRequest`].
    pub fn build(self) -> Result<ItemGroupsListCatalogRequest, BuildError> {
        Ok(ItemGroupsListCatalogRequest {})
    }
}
