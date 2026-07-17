pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersStatusesListRequest {}

impl PostV1PartnersStatusesListRequest {
    pub fn builder() -> PostV1PartnersStatusesListRequestBuilder {
        <PostV1PartnersStatusesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersStatusesListRequestBuilder {}

impl PostV1PartnersStatusesListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1PartnersStatusesListRequest`].
    pub fn build(self) -> Result<PostV1PartnersStatusesListRequest, BuildError> {
        Ok(PostV1PartnersStatusesListRequest {})
    }
}
