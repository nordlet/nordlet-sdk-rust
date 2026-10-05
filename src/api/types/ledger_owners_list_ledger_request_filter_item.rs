pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OwnersListLedgerRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: OwnersListLedgerRequestFilterItemOp,
    pub value: OwnersListLedgerRequestFilterItemValue,
}

impl OwnersListLedgerRequestFilterItem {
    pub fn builder() -> OwnersListLedgerRequestFilterItemBuilder {
        <OwnersListLedgerRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnersListLedgerRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<OwnersListLedgerRequestFilterItemOp>,
    value: Option<OwnersListLedgerRequestFilterItemValue>,
}

impl OwnersListLedgerRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: OwnersListLedgerRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: OwnersListLedgerRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnersListLedgerRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OwnersListLedgerRequestFilterItemBuilder::field)
    /// - [`op`](OwnersListLedgerRequestFilterItemBuilder::op)
    /// - [`value`](OwnersListLedgerRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<OwnersListLedgerRequestFilterItem, BuildError> {
        Ok(OwnersListLedgerRequestFilterItem {
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
