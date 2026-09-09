pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogUnitsListRequest {}

impl PostV1CatalogUnitsListRequest {
    pub fn builder() -> PostV1CatalogUnitsListRequestBuilder {
        <PostV1CatalogUnitsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogUnitsListRequestBuilder {}

impl PostV1CatalogUnitsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1CatalogUnitsListRequest`].
    pub fn build(self) -> Result<PostV1CatalogUnitsListRequest, BuildError> {
        Ok(PostV1CatalogUnitsListRequest {})
    }
}
