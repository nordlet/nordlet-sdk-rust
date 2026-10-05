pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersGetProductionRequest {
    #[serde(default)]
    pub id: String,
}

impl OrdersGetProductionRequest {
    pub fn builder() -> OrdersGetProductionRequestBuilder {
        <OrdersGetProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersGetProductionRequestBuilder {
    id: Option<String>,
}

impl OrdersGetProductionRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersGetProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersGetProductionRequestBuilder::id)
    pub fn build(self) -> Result<OrdersGetProductionRequest, BuildError> {
        Ok(OrdersGetProductionRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
