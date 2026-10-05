pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersListPurchasesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<OrdersListPurchasesRequestSortItemDir>,
}

impl OrdersListPurchasesRequestSortItem {
    pub fn builder() -> OrdersListPurchasesRequestSortItemBuilder {
        <OrdersListPurchasesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListPurchasesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<OrdersListPurchasesRequestSortItemDir>,
}

impl OrdersListPurchasesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: OrdersListPurchasesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersListPurchasesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrdersListPurchasesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<OrdersListPurchasesRequestSortItem, BuildError> {
        Ok(OrdersListPurchasesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
