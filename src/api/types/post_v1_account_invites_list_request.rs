pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesListRequest {}

impl PostV1AccountInvitesListRequest {
    pub fn builder() -> PostV1AccountInvitesListRequestBuilder {
        <PostV1AccountInvitesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesListRequestBuilder {}

impl PostV1AccountInvitesListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountInvitesListRequest`].
    pub fn build(self) -> Result<PostV1AccountInvitesListRequest, BuildError> {
        Ok(PostV1AccountInvitesListRequest {})
    }
}
