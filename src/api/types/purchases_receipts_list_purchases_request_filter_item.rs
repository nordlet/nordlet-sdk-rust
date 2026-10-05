pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReceiptsListPurchasesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ReceiptsListPurchasesRequestFilterItemOp,
    pub value: ReceiptsListPurchasesRequestFilterItemValue,
}

impl ReceiptsListPurchasesRequestFilterItem {
    pub fn builder() -> ReceiptsListPurchasesRequestFilterItemBuilder {
        <ReceiptsListPurchasesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsListPurchasesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ReceiptsListPurchasesRequestFilterItemOp>,
    value: Option<ReceiptsListPurchasesRequestFilterItemValue>,
}

impl ReceiptsListPurchasesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ReceiptsListPurchasesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ReceiptsListPurchasesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsListPurchasesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ReceiptsListPurchasesRequestFilterItemBuilder::field)
    /// - [`op`](ReceiptsListPurchasesRequestFilterItemBuilder::op)
    /// - [`value`](ReceiptsListPurchasesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ReceiptsListPurchasesRequestFilterItem, BuildError> {
        Ok(ReceiptsListPurchasesRequestFilterItem {
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
