pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EsVerifactuDeclaracionResponsableDeclarationsRequest {}

impl EsVerifactuDeclaracionResponsableDeclarationsRequest {
    pub fn builder() -> EsVerifactuDeclaracionResponsableDeclarationsRequestBuilder {
        <EsVerifactuDeclaracionResponsableDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EsVerifactuDeclaracionResponsableDeclarationsRequestBuilder {}

impl EsVerifactuDeclaracionResponsableDeclarationsRequestBuilder {
    /// Consumes the builder and constructs a [`EsVerifactuDeclaracionResponsableDeclarationsRequest`].
    pub fn build(self) -> Result<EsVerifactuDeclaracionResponsableDeclarationsRequest, BuildError> {
        Ok(EsVerifactuDeclaracionResponsableDeclarationsRequest {})
    }
}
