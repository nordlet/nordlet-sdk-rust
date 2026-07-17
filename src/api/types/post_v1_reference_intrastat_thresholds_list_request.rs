pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceIntrastatThresholdsListRequest {}

impl PostV1ReferenceIntrastatThresholdsListRequest {
    pub fn builder() -> PostV1ReferenceIntrastatThresholdsListRequestBuilder {
        <PostV1ReferenceIntrastatThresholdsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceIntrastatThresholdsListRequestBuilder {}

impl PostV1ReferenceIntrastatThresholdsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1ReferenceIntrastatThresholdsListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceIntrastatThresholdsListRequest, BuildError> {
        Ok(PostV1ReferenceIntrastatThresholdsListRequest {})
    }
}
