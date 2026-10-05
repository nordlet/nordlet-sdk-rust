pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtCountiesListReferenceRequest {}

impl LtCountiesListReferenceRequest {
    pub fn builder() -> LtCountiesListReferenceRequestBuilder {
        <LtCountiesListReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtCountiesListReferenceRequestBuilder {}

impl LtCountiesListReferenceRequestBuilder {
    /// Consumes the builder and constructs a [`LtCountiesListReferenceRequest`].
    pub fn build(self) -> Result<LtCountiesListReferenceRequest, BuildError> {
        Ok(LtCountiesListReferenceRequest {})
    }
}
