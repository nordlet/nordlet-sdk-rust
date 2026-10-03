pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCertificatesListRequest {}

impl PostV1DeclarationsCertificatesListRequest {
    pub fn builder() -> PostV1DeclarationsCertificatesListRequestBuilder {
        <PostV1DeclarationsCertificatesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCertificatesListRequestBuilder {}

impl PostV1DeclarationsCertificatesListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1DeclarationsCertificatesListRequest`].
    pub fn build(self) -> Result<PostV1DeclarationsCertificatesListRequest, BuildError> {
        Ok(PostV1DeclarationsCertificatesListRequest {})
    }
}
