pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtCountiesListRequest {}

impl PostV1ReferenceLtCountiesListRequest {
    pub fn builder() -> PostV1ReferenceLtCountiesListRequestBuilder {
        <PostV1ReferenceLtCountiesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtCountiesListRequestBuilder {}

impl PostV1ReferenceLtCountiesListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1ReferenceLtCountiesListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceLtCountiesListRequest, BuildError> {
        Ok(PostV1ReferenceLtCountiesListRequest {})
    }
}
