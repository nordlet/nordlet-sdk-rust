pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdvanceHoldersBalancesCashRequest {}

impl AdvanceHoldersBalancesCashRequest {
    pub fn builder() -> AdvanceHoldersBalancesCashRequestBuilder {
        <AdvanceHoldersBalancesCashRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdvanceHoldersBalancesCashRequestBuilder {}

impl AdvanceHoldersBalancesCashRequestBuilder {
    /// Consumes the builder and constructs a [`AdvanceHoldersBalancesCashRequest`].
    pub fn build(self) -> Result<AdvanceHoldersBalancesCashRequest, BuildError> {
        Ok(AdvanceHoldersBalancesCashRequest {})
    }
}
