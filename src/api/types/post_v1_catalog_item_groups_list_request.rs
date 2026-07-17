pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemGroupsListRequest {}

impl PostV1CatalogItemGroupsListRequest {
    pub fn builder() -> PostV1CatalogItemGroupsListRequestBuilder {
        <PostV1CatalogItemGroupsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemGroupsListRequestBuilder {}

impl PostV1CatalogItemGroupsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1CatalogItemGroupsListRequest`].
    pub fn build(self) -> Result<PostV1CatalogItemGroupsListRequest, BuildError> {
        Ok(PostV1CatalogItemGroupsListRequest {})
    }
}
