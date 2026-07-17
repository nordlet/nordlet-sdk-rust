pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountMeRequest {}

impl PostV1AccountMeRequest {
    pub fn builder() -> PostV1AccountMeRequestBuilder {
        <PostV1AccountMeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMeRequestBuilder {}

impl PostV1AccountMeRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountMeRequest`].
    pub fn build(self) -> Result<PostV1AccountMeRequest, BuildError> {
        Ok(PostV1AccountMeRequest {})
    }
}
