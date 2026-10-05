pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterGroupsCreateLedgerRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl CostCenterGroupsCreateLedgerRequest {
    pub fn builder() -> CostCenterGroupsCreateLedgerRequestBuilder {
        <CostCenterGroupsCreateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterGroupsCreateLedgerRequestBuilder {
    code: Option<String>,
    name: Option<String>,
}

impl CostCenterGroupsCreateLedgerRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CostCenterGroupsCreateLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CostCenterGroupsCreateLedgerRequestBuilder::code)
    /// - [`name`](CostCenterGroupsCreateLedgerRequestBuilder::name)
    pub fn build(self) -> Result<CostCenterGroupsCreateLedgerRequest, BuildError> {
        Ok(CostCenterGroupsCreateLedgerRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
