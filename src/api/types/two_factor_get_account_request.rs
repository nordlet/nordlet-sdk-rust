pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorGetAccountRequest {}

impl TwoFactorGetAccountRequest {
    pub fn builder() -> TwoFactorGetAccountRequestBuilder {
        <TwoFactorGetAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorGetAccountRequestBuilder {}

impl TwoFactorGetAccountRequestBuilder {
    /// Consumes the builder and constructs a [`TwoFactorGetAccountRequest`].
    pub fn build(self) -> Result<TwoFactorGetAccountRequest, BuildError> {
        Ok(TwoFactorGetAccountRequest {})
    }
}
