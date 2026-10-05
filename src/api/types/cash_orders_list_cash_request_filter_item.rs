pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdersListCashRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: OrdersListCashRequestFilterItemOp,
    pub value: OrdersListCashRequestFilterItemValue,
}

impl OrdersListCashRequestFilterItem {
    pub fn builder() -> OrdersListCashRequestFilterItemBuilder {
        <OrdersListCashRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListCashRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<OrdersListCashRequestFilterItemOp>,
    value: Option<OrdersListCashRequestFilterItemValue>,
}

impl OrdersListCashRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: OrdersListCashRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: OrdersListCashRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersListCashRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrdersListCashRequestFilterItemBuilder::field)
    /// - [`op`](OrdersListCashRequestFilterItemBuilder::op)
    /// - [`value`](OrdersListCashRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<OrdersListCashRequestFilterItem, BuildError> {
        Ok(OrdersListCashRequestFilterItem {
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
