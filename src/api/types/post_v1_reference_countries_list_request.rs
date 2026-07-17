pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceCountriesListRequest {}

impl PostV1ReferenceCountriesListRequest {
    pub fn builder() -> PostV1ReferenceCountriesListRequestBuilder {
        <PostV1ReferenceCountriesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCountriesListRequestBuilder {}

impl PostV1ReferenceCountriesListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1ReferenceCountriesListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceCountriesListRequest, BuildError> {
        Ok(PostV1ReferenceCountriesListRequest {})
    }
}
