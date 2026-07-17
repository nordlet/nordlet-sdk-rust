pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesProfileRequest {}

impl PostV1AccountCompaniesProfileRequest {
    pub fn builder() -> PostV1AccountCompaniesProfileRequestBuilder {
        <PostV1AccountCompaniesProfileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesProfileRequestBuilder {}

impl PostV1AccountCompaniesProfileRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountCompaniesProfileRequest`].
    pub fn build(self) -> Result<PostV1AccountCompaniesProfileRequest, BuildError> {
        Ok(PostV1AccountCompaniesProfileRequest {})
    }
}
