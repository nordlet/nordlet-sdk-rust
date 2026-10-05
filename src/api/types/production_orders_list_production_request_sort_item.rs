pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersListProductionRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<OrdersListProductionRequestSortItemDir>,
}

impl OrdersListProductionRequestSortItem {
    pub fn builder() -> OrdersListProductionRequestSortItemBuilder {
        <OrdersListProductionRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListProductionRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<OrdersListProductionRequestSortItemDir>,
}

impl OrdersListProductionRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: OrdersListProductionRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersListProductionRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrdersListProductionRequestSortItemBuilder::field)
    pub fn build(self) -> Result<OrdersListProductionRequestSortItem, BuildError> {
        Ok(OrdersListProductionRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
