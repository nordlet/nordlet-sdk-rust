pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuVatReturnPacksListRequest {}

impl PostV1DeclarationsEuVatReturnPacksListRequest {
    pub fn builder() -> PostV1DeclarationsEuVatReturnPacksListRequestBuilder {
        <PostV1DeclarationsEuVatReturnPacksListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuVatReturnPacksListRequestBuilder {}

impl PostV1DeclarationsEuVatReturnPacksListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1DeclarationsEuVatReturnPacksListRequest`].
    pub fn build(self) -> Result<PostV1DeclarationsEuVatReturnPacksListRequest, BuildError> {
        Ok(PostV1DeclarationsEuVatReturnPacksListRequest {})
    }
}
