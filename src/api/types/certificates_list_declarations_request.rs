pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CertificatesListDeclarationsRequest {}

impl CertificatesListDeclarationsRequest {
    pub fn builder() -> CertificatesListDeclarationsRequestBuilder {
        <CertificatesListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CertificatesListDeclarationsRequestBuilder {}

impl CertificatesListDeclarationsRequestBuilder {
    /// Consumes the builder and constructs a [`CertificatesListDeclarationsRequest`].
    pub fn build(self) -> Result<CertificatesListDeclarationsRequest, BuildError> {
        Ok(CertificatesListDeclarationsRequest {})
    }
}
