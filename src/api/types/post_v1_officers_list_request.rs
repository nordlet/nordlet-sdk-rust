pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OfficersListRequest {}

impl PostV1OfficersListRequest {
    pub fn builder() -> PostV1OfficersListRequestBuilder {
        <PostV1OfficersListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OfficersListRequestBuilder {}

impl PostV1OfficersListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1OfficersListRequest`].
    pub fn build(self) -> Result<PostV1OfficersListRequest, BuildError> {
        Ok(PostV1OfficersListRequest {})
    }
}
