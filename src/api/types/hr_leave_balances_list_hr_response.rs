pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaveBalancesListHrResponse {
    #[serde(default)]
    pub rows: Vec<LeaveBalancesListHrResponseRowsItem>,
}

impl LeaveBalancesListHrResponse {
    pub fn builder() -> LeaveBalancesListHrResponseBuilder {
        <LeaveBalancesListHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaveBalancesListHrResponseBuilder {
    rows: Option<Vec<LeaveBalancesListHrResponseRowsItem>>,
}

impl LeaveBalancesListHrResponseBuilder {
    pub fn rows(mut self, value: Vec<LeaveBalancesListHrResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LeaveBalancesListHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](LeaveBalancesListHrResponseBuilder::rows)
    pub fn build(self) -> Result<LeaveBalancesListHrResponse, BuildError> {
        Ok(LeaveBalancesListHrResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
