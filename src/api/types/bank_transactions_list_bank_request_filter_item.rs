pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionsListBankRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: TransactionsListBankRequestFilterItemOp,
    pub value: TransactionsListBankRequestFilterItemValue,
}

impl TransactionsListBankRequestFilterItem {
    pub fn builder() -> TransactionsListBankRequestFilterItemBuilder {
        <TransactionsListBankRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsListBankRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<TransactionsListBankRequestFilterItemOp>,
    value: Option<TransactionsListBankRequestFilterItemValue>,
}

impl TransactionsListBankRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: TransactionsListBankRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: TransactionsListBankRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsListBankRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](TransactionsListBankRequestFilterItemBuilder::field)
    /// - [`op`](TransactionsListBankRequestFilterItemBuilder::op)
    /// - [`value`](TransactionsListBankRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<TransactionsListBankRequestFilterItem, BuildError> {
        Ok(TransactionsListBankRequestFilterItem {
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
