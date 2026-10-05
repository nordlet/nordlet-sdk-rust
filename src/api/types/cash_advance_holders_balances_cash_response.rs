pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdvanceHoldersBalancesCashResponse {
    #[serde(default)]
    pub rows: Vec<AdvanceHoldersBalancesCashResponseRowsItem>,
}

impl AdvanceHoldersBalancesCashResponse {
    pub fn builder() -> AdvanceHoldersBalancesCashResponseBuilder {
        <AdvanceHoldersBalancesCashResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdvanceHoldersBalancesCashResponseBuilder {
    rows: Option<Vec<AdvanceHoldersBalancesCashResponseRowsItem>>,
}

impl AdvanceHoldersBalancesCashResponseBuilder {
    pub fn rows(mut self, value: Vec<AdvanceHoldersBalancesCashResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdvanceHoldersBalancesCashResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](AdvanceHoldersBalancesCashResponseBuilder::rows)
    pub fn build(self) -> Result<AdvanceHoldersBalancesCashResponse, BuildError> {
        Ok(AdvanceHoldersBalancesCashResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
