pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceTemplatesListSalesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: InvoiceTemplatesListSalesRequestFilterItemOp,
    pub value: InvoiceTemplatesListSalesRequestFilterItemValue,
}

impl InvoiceTemplatesListSalesRequestFilterItem {
    pub fn builder() -> InvoiceTemplatesListSalesRequestFilterItemBuilder {
        <InvoiceTemplatesListSalesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceTemplatesListSalesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<InvoiceTemplatesListSalesRequestFilterItemOp>,
    value: Option<InvoiceTemplatesListSalesRequestFilterItemValue>,
}

impl InvoiceTemplatesListSalesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: InvoiceTemplatesListSalesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: InvoiceTemplatesListSalesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceTemplatesListSalesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InvoiceTemplatesListSalesRequestFilterItemBuilder::field)
    /// - [`op`](InvoiceTemplatesListSalesRequestFilterItemBuilder::op)
    /// - [`value`](InvoiceTemplatesListSalesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<InvoiceTemplatesListSalesRequestFilterItem, BuildError> {
        Ok(InvoiceTemplatesListSalesRequestFilterItem {
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
