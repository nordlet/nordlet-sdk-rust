pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterGroupsDeleteLedgerRequest {
    #[serde(default)]
    pub id: String,
}

impl CostCenterGroupsDeleteLedgerRequest {
    pub fn builder() -> CostCenterGroupsDeleteLedgerRequestBuilder {
        <CostCenterGroupsDeleteLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterGroupsDeleteLedgerRequestBuilder {
    id: Option<String>,
}

impl CostCenterGroupsDeleteLedgerRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CostCenterGroupsDeleteLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CostCenterGroupsDeleteLedgerRequestBuilder::id)
    pub fn build(self) -> Result<CostCenterGroupsDeleteLedgerRequest, BuildError> {
        Ok(CostCenterGroupsDeleteLedgerRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
