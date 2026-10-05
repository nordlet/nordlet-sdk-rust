pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersListEcommerceRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<OrdersListEcommerceRequestSortItemDir>,
}

impl OrdersListEcommerceRequestSortItem {
    pub fn builder() -> OrdersListEcommerceRequestSortItemBuilder {
        <OrdersListEcommerceRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListEcommerceRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<OrdersListEcommerceRequestSortItemDir>,
}

impl OrdersListEcommerceRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: OrdersListEcommerceRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersListEcommerceRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrdersListEcommerceRequestSortItemBuilder::field)
    pub fn build(self) -> Result<OrdersListEcommerceRequestSortItem, BuildError> {
        Ok(OrdersListEcommerceRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
