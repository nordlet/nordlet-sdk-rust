pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersReserveEcommerceRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl OrdersReserveEcommerceRequest {
    pub fn builder() -> OrdersReserveEcommerceRequestBuilder {
        <OrdersReserveEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersReserveEcommerceRequestBuilder {
    id: Option<String>,
    warehouse_id: Option<String>,
}

impl OrdersReserveEcommerceRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersReserveEcommerceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersReserveEcommerceRequestBuilder::id)
    pub fn build(self) -> Result<OrdersReserveEcommerceRequest, BuildError> {
        Ok(OrdersReserveEcommerceRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            warehouse_id: self.warehouse_id,
        })
    }
}
