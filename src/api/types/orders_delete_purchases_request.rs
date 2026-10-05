pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersDeletePurchasesRequest {
    #[serde(default)]
    pub id: String,
}

impl OrdersDeletePurchasesRequest {
    pub fn builder() -> OrdersDeletePurchasesRequestBuilder {
        <OrdersDeletePurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersDeletePurchasesRequestBuilder {
    id: Option<String>,
}

impl OrdersDeletePurchasesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersDeletePurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersDeletePurchasesRequestBuilder::id)
    pub fn build(self) -> Result<OrdersDeletePurchasesRequest, BuildError> {
        Ok(OrdersDeletePurchasesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
