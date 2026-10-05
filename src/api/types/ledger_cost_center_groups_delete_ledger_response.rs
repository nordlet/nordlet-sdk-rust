pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterGroupsDeleteLedgerResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl CostCenterGroupsDeleteLedgerResponse {
    pub fn builder() -> CostCenterGroupsDeleteLedgerResponseBuilder {
        <CostCenterGroupsDeleteLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterGroupsDeleteLedgerResponseBuilder {
    deleted: Option<bool>,
}

impl CostCenterGroupsDeleteLedgerResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CostCenterGroupsDeleteLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](CostCenterGroupsDeleteLedgerResponseBuilder::deleted)
    pub fn build(self) -> Result<CostCenterGroupsDeleteLedgerResponse, BuildError> {
        Ok(CostCenterGroupsDeleteLedgerResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
