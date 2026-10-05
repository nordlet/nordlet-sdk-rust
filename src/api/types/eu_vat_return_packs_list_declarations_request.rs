pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuVatReturnPacksListDeclarationsRequest {}

impl EuVatReturnPacksListDeclarationsRequest {
    pub fn builder() -> EuVatReturnPacksListDeclarationsRequestBuilder {
        <EuVatReturnPacksListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatReturnPacksListDeclarationsRequestBuilder {}

impl EuVatReturnPacksListDeclarationsRequestBuilder {
    /// Consumes the builder and constructs a [`EuVatReturnPacksListDeclarationsRequest`].
    pub fn build(self) -> Result<EuVatReturnPacksListDeclarationsRequest, BuildError> {
        Ok(EuVatReturnPacksListDeclarationsRequest {})
    }
}
