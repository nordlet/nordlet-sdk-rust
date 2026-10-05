pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtRegionsListReferenceRequest {}

impl LtRegionsListReferenceRequest {
    pub fn builder() -> LtRegionsListReferenceRequestBuilder {
        <LtRegionsListReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtRegionsListReferenceRequestBuilder {}

impl LtRegionsListReferenceRequestBuilder {
    /// Consumes the builder and constructs a [`LtRegionsListReferenceRequest`].
    pub fn build(self) -> Result<LtRegionsListReferenceRequest, BuildError> {
        Ok(LtRegionsListReferenceRequest {})
    }
}
