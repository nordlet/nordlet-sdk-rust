pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterGroupsUpdateLedgerRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CostCenterGroupsUpdateLedgerRequest {
    pub fn builder() -> CostCenterGroupsUpdateLedgerRequestBuilder {
        <CostCenterGroupsUpdateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterGroupsUpdateLedgerRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl CostCenterGroupsUpdateLedgerRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CostCenterGroupsUpdateLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CostCenterGroupsUpdateLedgerRequestBuilder::id)
    pub fn build(self) -> Result<CostCenterGroupsUpdateLedgerRequest, BuildError> {
        Ok(CostCenterGroupsUpdateLedgerRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
        })
    }
}
