pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdersListProductionRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: OrdersListProductionRequestFilterItemOp,
    pub value: OrdersListProductionRequestFilterItemValue,
}

impl OrdersListProductionRequestFilterItem {
    pub fn builder() -> OrdersListProductionRequestFilterItemBuilder {
        <OrdersListProductionRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListProductionRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<OrdersListProductionRequestFilterItemOp>,
    value: Option<OrdersListProductionRequestFilterItemValue>,
}

impl OrdersListProductionRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: OrdersListProductionRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: OrdersListProductionRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersListProductionRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrdersListProductionRequestFilterItemBuilder::field)
    /// - [`op`](OrdersListProductionRequestFilterItemBuilder::op)
    /// - [`value`](OrdersListProductionRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<OrdersListProductionRequestFilterItem, BuildError> {
        Ok(OrdersListProductionRequestFilterItem {
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
