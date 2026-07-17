pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersGroupsListRequest {}

impl PostV1PartnersGroupsListRequest {
    pub fn builder() -> PostV1PartnersGroupsListRequestBuilder {
        <PostV1PartnersGroupsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersGroupsListRequestBuilder {}

impl PostV1PartnersGroupsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1PartnersGroupsListRequest`].
    pub fn build(self) -> Result<PostV1PartnersGroupsListRequest, BuildError> {
        Ok(PostV1PartnersGroupsListRequest {})
    }
}
