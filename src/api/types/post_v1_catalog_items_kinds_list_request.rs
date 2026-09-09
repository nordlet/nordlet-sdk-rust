pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsKindsListRequest {}

impl PostV1CatalogItemsKindsListRequest {
    pub fn builder() -> PostV1CatalogItemsKindsListRequestBuilder {
        <PostV1CatalogItemsKindsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsKindsListRequestBuilder {}

impl PostV1CatalogItemsKindsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1CatalogItemsKindsListRequest`].
    pub fn build(self) -> Result<PostV1CatalogItemsKindsListRequest, BuildError> {
        Ok(PostV1CatalogItemsKindsListRequest {})
    }
}
