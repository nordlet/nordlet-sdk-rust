pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeriodsListLedgerRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PeriodsListLedgerRequestFilterItemOp,
    pub value: PeriodsListLedgerRequestFilterItemValue,
}

impl PeriodsListLedgerRequestFilterItem {
    pub fn builder() -> PeriodsListLedgerRequestFilterItemBuilder {
        <PeriodsListLedgerRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeriodsListLedgerRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PeriodsListLedgerRequestFilterItemOp>,
    value: Option<PeriodsListLedgerRequestFilterItemValue>,
}

impl PeriodsListLedgerRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PeriodsListLedgerRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PeriodsListLedgerRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeriodsListLedgerRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PeriodsListLedgerRequestFilterItemBuilder::field)
    /// - [`op`](PeriodsListLedgerRequestFilterItemBuilder::op)
    /// - [`value`](PeriodsListLedgerRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PeriodsListLedgerRequestFilterItem, BuildError> {
        Ok(PeriodsListLedgerRequestFilterItem {
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
