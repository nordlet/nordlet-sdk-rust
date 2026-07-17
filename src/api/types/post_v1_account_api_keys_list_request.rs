pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountApiKeysListRequest {}

impl PostV1AccountApiKeysListRequest {
    pub fn builder() -> PostV1AccountApiKeysListRequestBuilder {
        <PostV1AccountApiKeysListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountApiKeysListRequestBuilder {}

impl PostV1AccountApiKeysListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountApiKeysListRequest`].
    pub fn build(self) -> Result<PostV1AccountApiKeysListRequest, BuildError> {
        Ok(PostV1AccountApiKeysListRequest {})
    }
}
