pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CashAdvanceHoldersBalancesRequest {}

impl PostV1CashAdvanceHoldersBalancesRequest {
    pub fn builder() -> PostV1CashAdvanceHoldersBalancesRequestBuilder {
        <PostV1CashAdvanceHoldersBalancesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CashAdvanceHoldersBalancesRequestBuilder {}

impl PostV1CashAdvanceHoldersBalancesRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1CashAdvanceHoldersBalancesRequest`].
    pub fn build(self) -> Result<PostV1CashAdvanceHoldersBalancesRequest, BuildError> {
        Ok(PostV1CashAdvanceHoldersBalancesRequest {})
    }
}
