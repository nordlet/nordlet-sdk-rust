pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntrastatThresholdsListReferenceRequest {}

impl IntrastatThresholdsListReferenceRequest {
    pub fn builder() -> IntrastatThresholdsListReferenceRequestBuilder {
        <IntrastatThresholdsListReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntrastatThresholdsListReferenceRequestBuilder {}

impl IntrastatThresholdsListReferenceRequestBuilder {
    /// Consumes the builder and constructs a [`IntrastatThresholdsListReferenceRequest`].
    pub fn build(self) -> Result<IntrastatThresholdsListReferenceRequest, BuildError> {
        Ok(IntrastatThresholdsListReferenceRequest {})
    }
}
