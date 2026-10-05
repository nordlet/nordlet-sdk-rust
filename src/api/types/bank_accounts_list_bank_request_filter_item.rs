pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountsListBankRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: AccountsListBankRequestFilterItemOp,
    pub value: AccountsListBankRequestFilterItemValue,
}

impl AccountsListBankRequestFilterItem {
    pub fn builder() -> AccountsListBankRequestFilterItemBuilder {
        <AccountsListBankRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsListBankRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<AccountsListBankRequestFilterItemOp>,
    value: Option<AccountsListBankRequestFilterItemValue>,
}

impl AccountsListBankRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: AccountsListBankRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: AccountsListBankRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsListBankRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AccountsListBankRequestFilterItemBuilder::field)
    /// - [`op`](AccountsListBankRequestFilterItemBuilder::op)
    /// - [`value`](AccountsListBankRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<AccountsListBankRequestFilterItem, BuildError> {
        Ok(AccountsListBankRequestFilterItem {
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
