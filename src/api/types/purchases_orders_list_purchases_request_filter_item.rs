pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdersListPurchasesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: OrdersListPurchasesRequestFilterItemOp,
    pub value: OrdersListPurchasesRequestFilterItemValue,
}

impl OrdersListPurchasesRequestFilterItem {
    pub fn builder() -> OrdersListPurchasesRequestFilterItemBuilder {
        <OrdersListPurchasesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListPurchasesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<OrdersListPurchasesRequestFilterItemOp>,
    value: Option<OrdersListPurchasesRequestFilterItemValue>,
}

impl OrdersListPurchasesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: OrdersListPurchasesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: OrdersListPurchasesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersListPurchasesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrdersListPurchasesRequestFilterItemBuilder::field)
    /// - [`op`](OrdersListPurchasesRequestFilterItemBuilder::op)
    /// - [`value`](OrdersListPurchasesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<OrdersListPurchasesRequestFilterItem, BuildError> {
        Ok(OrdersListPurchasesRequestFilterItem {
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
