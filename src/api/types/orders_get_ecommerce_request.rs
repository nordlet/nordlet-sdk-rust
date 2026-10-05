pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersGetEcommerceRequest {
    #[serde(default)]
    pub id: String,
}

impl OrdersGetEcommerceRequest {
    pub fn builder() -> OrdersGetEcommerceRequestBuilder {
        <OrdersGetEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersGetEcommerceRequestBuilder {
    id: Option<String>,
}

impl OrdersGetEcommerceRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersGetEcommerceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersGetEcommerceRequestBuilder::id)
    pub fn build(self) -> Result<OrdersGetEcommerceRequest, BuildError> {
        Ok(OrdersGetEcommerceRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
