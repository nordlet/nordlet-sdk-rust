pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorSetupAccountRequest {}

impl TwoFactorSetupAccountRequest {
    pub fn builder() -> TwoFactorSetupAccountRequestBuilder {
        <TwoFactorSetupAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorSetupAccountRequestBuilder {}

impl TwoFactorSetupAccountRequestBuilder {
    /// Consumes the builder and constructs a [`TwoFactorSetupAccountRequest`].
    pub fn build(self) -> Result<TwoFactorSetupAccountRequest, BuildError> {
        Ok(TwoFactorSetupAccountRequest {})
    }
}
