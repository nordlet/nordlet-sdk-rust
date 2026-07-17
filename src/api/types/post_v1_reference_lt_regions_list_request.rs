pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtRegionsListRequest {}

impl PostV1ReferenceLtRegionsListRequest {
    pub fn builder() -> PostV1ReferenceLtRegionsListRequestBuilder {
        <PostV1ReferenceLtRegionsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtRegionsListRequestBuilder {}

impl PostV1ReferenceLtRegionsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1ReferenceLtRegionsListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceLtRegionsListRequest, BuildError> {
        Ok(PostV1ReferenceLtRegionsListRequest {})
    }
}
