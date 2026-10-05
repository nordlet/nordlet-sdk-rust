pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersGetCashRequest {
    #[serde(default)]
    pub id: String,
}

impl OrdersGetCashRequest {
    pub fn builder() -> OrdersGetCashRequestBuilder {
        <OrdersGetCashRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersGetCashRequestBuilder {
    id: Option<String>,
}

impl OrdersGetCashRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersGetCashRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersGetCashRequestBuilder::id)
    pub fn build(self) -> Result<OrdersGetCashRequest, BuildError> {
        Ok(OrdersGetCashRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
