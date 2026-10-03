pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEsVerifactuDeclaracionResponsableRequest {}

impl PostV1DeclarationsEsVerifactuDeclaracionResponsableRequest {
    pub fn builder() -> PostV1DeclarationsEsVerifactuDeclaracionResponsableRequestBuilder {
        <PostV1DeclarationsEsVerifactuDeclaracionResponsableRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEsVerifactuDeclaracionResponsableRequestBuilder {}

impl PostV1DeclarationsEsVerifactuDeclaracionResponsableRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1DeclarationsEsVerifactuDeclaracionResponsableRequest`].
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEsVerifactuDeclaracionResponsableRequest, BuildError> {
        Ok(PostV1DeclarationsEsVerifactuDeclaracionResponsableRequest {})
    }
}
