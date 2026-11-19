pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbHmrcAuthorizeUrlGetDeclarationsRequest {}

impl GbHmrcAuthorizeUrlGetDeclarationsRequest {
    pub fn builder() -> GbHmrcAuthorizeUrlGetDeclarationsRequestBuilder {
        <GbHmrcAuthorizeUrlGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbHmrcAuthorizeUrlGetDeclarationsRequestBuilder {}

impl GbHmrcAuthorizeUrlGetDeclarationsRequestBuilder {
    /// Consumes the builder and constructs a [`GbHmrcAuthorizeUrlGetDeclarationsRequest`].
    pub fn build(self) -> Result<GbHmrcAuthorizeUrlGetDeclarationsRequest, BuildError> {
        Ok(GbHmrcAuthorizeUrlGetDeclarationsRequest {})
    }
}
