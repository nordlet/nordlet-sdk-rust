pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostCenterGroupsListLedgerRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: CostCenterGroupsListLedgerRequestFilterItemOp,
    pub value: CostCenterGroupsListLedgerRequestFilterItemValue,
}

impl CostCenterGroupsListLedgerRequestFilterItem {
    pub fn builder() -> CostCenterGroupsListLedgerRequestFilterItemBuilder {
        <CostCenterGroupsListLedgerRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterGroupsListLedgerRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<CostCenterGroupsListLedgerRequestFilterItemOp>,
    value: Option<CostCenterGroupsListLedgerRequestFilterItemValue>,
}

impl CostCenterGroupsListLedgerRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: CostCenterGroupsListLedgerRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: CostCenterGroupsListLedgerRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CostCenterGroupsListLedgerRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CostCenterGroupsListLedgerRequestFilterItemBuilder::field)
    /// - [`op`](CostCenterGroupsListLedgerRequestFilterItemBuilder::op)
    /// - [`value`](CostCenterGroupsListLedgerRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<CostCenterGroupsListLedgerRequestFilterItem, BuildError> {
        Ok(CostCenterGroupsListLedgerRequestFilterItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
