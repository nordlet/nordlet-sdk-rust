pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdersListEcommerceRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: OrdersListEcommerceRequestFilterItemOp,
    pub value: OrdersListEcommerceRequestFilterItemValue,
}

impl OrdersListEcommerceRequestFilterItem {
    pub fn builder() -> OrdersListEcommerceRequestFilterItemBuilder {
        <OrdersListEcommerceRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListEcommerceRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<OrdersListEcommerceRequestFilterItemOp>,
    value: Option<OrdersListEcommerceRequestFilterItemValue>,
}

impl OrdersListEcommerceRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: OrdersListEcommerceRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: OrdersListEcommerceRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersListEcommerceRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrdersListEcommerceRequestFilterItemBuilder::field)
    /// - [`op`](OrdersListEcommerceRequestFilterItemBuilder::op)
    /// - [`value`](OrdersListEcommerceRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<OrdersListEcommerceRequestFilterItem, BuildError> {
        Ok(OrdersListEcommerceRequestFilterItem {
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
