pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostCentersListLedgerRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: CostCentersListLedgerRequestFilterItemOp,
    pub value: CostCentersListLedgerRequestFilterItemValue,
}

impl CostCentersListLedgerRequestFilterItem {
    pub fn builder() -> CostCentersListLedgerRequestFilterItemBuilder {
        <CostCentersListLedgerRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCentersListLedgerRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<CostCentersListLedgerRequestFilterItemOp>,
    value: Option<CostCentersListLedgerRequestFilterItemValue>,
}

impl CostCentersListLedgerRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: CostCentersListLedgerRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: CostCentersListLedgerRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CostCentersListLedgerRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CostCentersListLedgerRequestFilterItemBuilder::field)
    /// - [`op`](CostCentersListLedgerRequestFilterItemBuilder::op)
    /// - [`value`](CostCentersListLedgerRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<CostCentersListLedgerRequestFilterItem, BuildError> {
        Ok(CostCentersListLedgerRequestFilterItem {
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
