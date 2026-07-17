pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountMembersListRequest {}

impl PostV1AccountMembersListRequest {
    pub fn builder() -> PostV1AccountMembersListRequestBuilder {
        <PostV1AccountMembersListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMembersListRequestBuilder {}

impl PostV1AccountMembersListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountMembersListRequest`].
    pub fn build(self) -> Result<PostV1AccountMembersListRequest, BuildError> {
        Ok(PostV1AccountMembersListRequest {})
    }
}
