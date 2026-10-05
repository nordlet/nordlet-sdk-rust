pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoicesListSalesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: InvoicesListSalesRequestFilterItemOp,
    pub value: InvoicesListSalesRequestFilterItemValue,
}

impl InvoicesListSalesRequestFilterItem {
    pub fn builder() -> InvoicesListSalesRequestFilterItemBuilder {
        <InvoicesListSalesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesListSalesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<InvoicesListSalesRequestFilterItemOp>,
    value: Option<InvoicesListSalesRequestFilterItemValue>,
}

impl InvoicesListSalesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: InvoicesListSalesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: InvoicesListSalesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesListSalesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InvoicesListSalesRequestFilterItemBuilder::field)
    /// - [`op`](InvoicesListSalesRequestFilterItemBuilder::op)
    /// - [`value`](InvoicesListSalesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<InvoicesListSalesRequestFilterItem, BuildError> {
        Ok(InvoicesListSalesRequestFilterItem {
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
