pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersListCashRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<OrdersListCashRequestSortItemDir>,
}

impl OrdersListCashRequestSortItem {
    pub fn builder() -> OrdersListCashRequestSortItemBuilder {
        <OrdersListCashRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListCashRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<OrdersListCashRequestSortItemDir>,
}

impl OrdersListCashRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: OrdersListCashRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersListCashRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrdersListCashRequestSortItemBuilder::field)
    pub fn build(self) -> Result<OrdersListCashRequestSortItem, BuildError> {
        Ok(OrdersListCashRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
