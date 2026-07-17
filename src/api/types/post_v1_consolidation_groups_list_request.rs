pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationGroupsListRequest {}

impl PostV1ConsolidationGroupsListRequest {
    pub fn builder() -> PostV1ConsolidationGroupsListRequestBuilder {
        <PostV1ConsolidationGroupsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationGroupsListRequestBuilder {}

impl PostV1ConsolidationGroupsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1ConsolidationGroupsListRequest`].
    pub fn build(self) -> Result<PostV1ConsolidationGroupsListRequest, BuildError> {
        Ok(PostV1ConsolidationGroupsListRequest {})
    }
}
