pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoicesListPurchasesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: InvoicesListPurchasesRequestFilterItemOp,
    pub value: InvoicesListPurchasesRequestFilterItemValue,
}

impl InvoicesListPurchasesRequestFilterItem {
    pub fn builder() -> InvoicesListPurchasesRequestFilterItemBuilder {
        <InvoicesListPurchasesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesListPurchasesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<InvoicesListPurchasesRequestFilterItemOp>,
    value: Option<InvoicesListPurchasesRequestFilterItemValue>,
}

impl InvoicesListPurchasesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: InvoicesListPurchasesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: InvoicesListPurchasesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesListPurchasesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InvoicesListPurchasesRequestFilterItemBuilder::field)
    /// - [`op`](InvoicesListPurchasesRequestFilterItemBuilder::op)
    /// - [`value`](InvoicesListPurchasesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<InvoicesListPurchasesRequestFilterItem, BuildError> {
        Ok(InvoicesListPurchasesRequestFilterItem {
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
