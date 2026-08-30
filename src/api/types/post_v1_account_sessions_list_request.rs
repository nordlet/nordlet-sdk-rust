pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountSessionsListRequest {}

impl PostV1AccountSessionsListRequest {
    pub fn builder() -> PostV1AccountSessionsListRequestBuilder {
        <PostV1AccountSessionsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountSessionsListRequestBuilder {}

impl PostV1AccountSessionsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountSessionsListRequest`].
    pub fn build(self) -> Result<PostV1AccountSessionsListRequest, BuildError> {
        Ok(PostV1AccountSessionsListRequest {})
    }
}
