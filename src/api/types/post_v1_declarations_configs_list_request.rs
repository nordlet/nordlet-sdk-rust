pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsConfigsListRequest {}

impl PostV1DeclarationsConfigsListRequest {
    pub fn builder() -> PostV1DeclarationsConfigsListRequestBuilder {
        <PostV1DeclarationsConfigsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsConfigsListRequestBuilder {}

impl PostV1DeclarationsConfigsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1DeclarationsConfigsListRequest`].
    pub fn build(self) -> Result<PostV1DeclarationsConfigsListRequest, BuildError> {
        Ok(PostV1DeclarationsConfigsListRequest {})
    }
}
