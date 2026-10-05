pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeAccountRequest {}

impl MeAccountRequest {
    pub fn builder() -> MeAccountRequestBuilder {
        <MeAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeAccountRequestBuilder {}

impl MeAccountRequestBuilder {
    /// Consumes the builder and constructs a [`MeAccountRequest`].
    pub fn build(self) -> Result<MeAccountRequest, BuildError> {
        Ok(MeAccountRequest {})
    }
}
