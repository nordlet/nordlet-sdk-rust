pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecurringInvoicesListSalesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: RecurringInvoicesListSalesRequestFilterItemOp,
    pub value: RecurringInvoicesListSalesRequestFilterItemValue,
}

impl RecurringInvoicesListSalesRequestFilterItem {
    pub fn builder() -> RecurringInvoicesListSalesRequestFilterItemBuilder {
        <RecurringInvoicesListSalesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesListSalesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<RecurringInvoicesListSalesRequestFilterItemOp>,
    value: Option<RecurringInvoicesListSalesRequestFilterItemValue>,
}

impl RecurringInvoicesListSalesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: RecurringInvoicesListSalesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: RecurringInvoicesListSalesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesListSalesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RecurringInvoicesListSalesRequestFilterItemBuilder::field)
    /// - [`op`](RecurringInvoicesListSalesRequestFilterItemBuilder::op)
    /// - [`value`](RecurringInvoicesListSalesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<RecurringInvoicesListSalesRequestFilterItem, BuildError> {
        Ok(RecurringInvoicesListSalesRequestFilterItem {
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
