pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BankAccountsListPartnersRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: BankAccountsListPartnersRequestFilterItemOp,
    pub value: BankAccountsListPartnersRequestFilterItemValue,
}

impl BankAccountsListPartnersRequestFilterItem {
    pub fn builder() -> BankAccountsListPartnersRequestFilterItemBuilder {
        <BankAccountsListPartnersRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountsListPartnersRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<BankAccountsListPartnersRequestFilterItemOp>,
    value: Option<BankAccountsListPartnersRequestFilterItemValue>,
}

impl BankAccountsListPartnersRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: BankAccountsListPartnersRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: BankAccountsListPartnersRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BankAccountsListPartnersRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](BankAccountsListPartnersRequestFilterItemBuilder::field)
    /// - [`op`](BankAccountsListPartnersRequestFilterItemBuilder::op)
    /// - [`value`](BankAccountsListPartnersRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<BankAccountsListPartnersRequestFilterItem, BuildError> {
        Ok(BankAccountsListPartnersRequestFilterItem {
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
