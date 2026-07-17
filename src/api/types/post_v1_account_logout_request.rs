pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountLogoutRequest {}

impl PostV1AccountLogoutRequest {
    pub fn builder() -> PostV1AccountLogoutRequestBuilder {
        <PostV1AccountLogoutRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountLogoutRequestBuilder {}

impl PostV1AccountLogoutRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountLogoutRequest`].
    pub fn build(self) -> Result<PostV1AccountLogoutRequest, BuildError> {
        Ok(PostV1AccountLogoutRequest {})
    }
}
