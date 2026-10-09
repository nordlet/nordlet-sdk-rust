pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpenseReportsListCashRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ExpenseReportsListCashRequestFilterItemOp,
    pub value: ExpenseReportsListCashRequestFilterItemValue,
}

impl ExpenseReportsListCashRequestFilterItem {
    pub fn builder() -> ExpenseReportsListCashRequestFilterItemBuilder {
        <ExpenseReportsListCashRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpenseReportsListCashRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ExpenseReportsListCashRequestFilterItemOp>,
    value: Option<ExpenseReportsListCashRequestFilterItemValue>,
}

impl ExpenseReportsListCashRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ExpenseReportsListCashRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ExpenseReportsListCashRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpenseReportsListCashRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ExpenseReportsListCashRequestFilterItemBuilder::field)
    /// - [`op`](ExpenseReportsListCashRequestFilterItemBuilder::op)
    /// - [`value`](ExpenseReportsListCashRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ExpenseReportsListCashRequestFilterItem, BuildError> {
        Ok(ExpenseReportsListCashRequestFilterItem {
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
