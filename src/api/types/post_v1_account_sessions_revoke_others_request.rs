pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountSessionsRevokeOthersRequest {}

impl PostV1AccountSessionsRevokeOthersRequest {
    pub fn builder() -> PostV1AccountSessionsRevokeOthersRequestBuilder {
        <PostV1AccountSessionsRevokeOthersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountSessionsRevokeOthersRequestBuilder {}

impl PostV1AccountSessionsRevokeOthersRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountSessionsRevokeOthersRequest`].
    pub fn build(self) -> Result<PostV1AccountSessionsRevokeOthersRequest, BuildError> {
        Ok(PostV1AccountSessionsRevokeOthersRequest {})
    }
}
