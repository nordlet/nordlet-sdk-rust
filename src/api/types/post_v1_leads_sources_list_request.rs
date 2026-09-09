pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsSourcesListRequest {}

impl PostV1LeadsSourcesListRequest {
    pub fn builder() -> PostV1LeadsSourcesListRequestBuilder {
        <PostV1LeadsSourcesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsSourcesListRequestBuilder {}

impl PostV1LeadsSourcesListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1LeadsSourcesListRequest`].
    pub fn build(self) -> Result<PostV1LeadsSourcesListRequest, BuildError> {
        Ok(PostV1LeadsSourcesListRequest {})
    }
}
