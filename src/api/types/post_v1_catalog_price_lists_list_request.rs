pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsListRequest {}

impl PostV1CatalogPriceListsListRequest {
    pub fn builder() -> PostV1CatalogPriceListsListRequestBuilder {
        <PostV1CatalogPriceListsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsListRequestBuilder {}

impl PostV1CatalogPriceListsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsListRequest`].
    pub fn build(self) -> Result<PostV1CatalogPriceListsListRequest, BuildError> {
        Ok(PostV1CatalogPriceListsListRequest {})
    }
}
