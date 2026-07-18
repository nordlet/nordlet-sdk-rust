pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeThresholdsListRequest {}

impl PostV1DeclarationsEuSmeThresholdsListRequest {
    pub fn builder() -> PostV1DeclarationsEuSmeThresholdsListRequestBuilder {
        <PostV1DeclarationsEuSmeThresholdsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeThresholdsListRequestBuilder {}

impl PostV1DeclarationsEuSmeThresholdsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeThresholdsListRequest`].
    pub fn build(self) -> Result<PostV1DeclarationsEuSmeThresholdsListRequest, BuildError> {
        Ok(PostV1DeclarationsEuSmeThresholdsListRequest {})
    }
}
