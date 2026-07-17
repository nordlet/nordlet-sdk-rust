pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CashAdvanceHoldersBalancesResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CashAdvanceHoldersBalancesResponseRowsItem>,
}

impl PostV1CashAdvanceHoldersBalancesResponse {
    pub fn builder() -> PostV1CashAdvanceHoldersBalancesResponseBuilder {
        <PostV1CashAdvanceHoldersBalancesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CashAdvanceHoldersBalancesResponseBuilder {
    rows: Option<Vec<PostV1CashAdvanceHoldersBalancesResponseRowsItem>>,
}

impl PostV1CashAdvanceHoldersBalancesResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CashAdvanceHoldersBalancesResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CashAdvanceHoldersBalancesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CashAdvanceHoldersBalancesResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CashAdvanceHoldersBalancesResponse, BuildError> {
        Ok(PostV1CashAdvanceHoldersBalancesResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
