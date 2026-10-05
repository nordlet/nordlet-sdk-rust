pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountsListLedgerRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: AccountsListLedgerRequestFilterItemOp,
    pub value: AccountsListLedgerRequestFilterItemValue,
}

impl AccountsListLedgerRequestFilterItem {
    pub fn builder() -> AccountsListLedgerRequestFilterItemBuilder {
        <AccountsListLedgerRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsListLedgerRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<AccountsListLedgerRequestFilterItemOp>,
    value: Option<AccountsListLedgerRequestFilterItemValue>,
}

impl AccountsListLedgerRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: AccountsListLedgerRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: AccountsListLedgerRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsListLedgerRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AccountsListLedgerRequestFilterItemBuilder::field)
    /// - [`op`](AccountsListLedgerRequestFilterItemBuilder::op)
    /// - [`value`](AccountsListLedgerRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<AccountsListLedgerRequestFilterItem, BuildError> {
        Ok(AccountsListLedgerRequestFilterItem {
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
