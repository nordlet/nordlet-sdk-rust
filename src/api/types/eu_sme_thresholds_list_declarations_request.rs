pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdsListDeclarationsRequest {}

impl EuSmeThresholdsListDeclarationsRequest {
    pub fn builder() -> EuSmeThresholdsListDeclarationsRequestBuilder {
        <EuSmeThresholdsListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdsListDeclarationsRequestBuilder {}

impl EuSmeThresholdsListDeclarationsRequestBuilder {
    /// Consumes the builder and constructs a [`EuSmeThresholdsListDeclarationsRequest`].
    pub fn build(self) -> Result<EuSmeThresholdsListDeclarationsRequest, BuildError> {
        Ok(EuSmeThresholdsListDeclarationsRequest {})
    }
}
