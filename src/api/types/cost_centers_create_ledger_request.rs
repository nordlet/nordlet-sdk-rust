pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCentersCreateLedgerRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl CostCentersCreateLedgerRequest {
    pub fn builder() -> CostCentersCreateLedgerRequestBuilder {
        <CostCentersCreateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCentersCreateLedgerRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    group_id: Option<String>,
}

impl CostCentersCreateLedgerRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CostCentersCreateLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CostCentersCreateLedgerRequestBuilder::code)
    /// - [`name`](CostCentersCreateLedgerRequestBuilder::name)
    pub fn build(self) -> Result<CostCentersCreateLedgerRequest, BuildError> {
        Ok(CostCentersCreateLedgerRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            group_id: self.group_id,
        })
    }
}
