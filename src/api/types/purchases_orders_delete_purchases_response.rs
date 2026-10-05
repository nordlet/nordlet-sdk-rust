pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersDeletePurchasesResponse {
    #[serde(default)]
    pub id: String,
}

impl OrdersDeletePurchasesResponse {
    pub fn builder() -> OrdersDeletePurchasesResponseBuilder {
        <OrdersDeletePurchasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersDeletePurchasesResponseBuilder {
    id: Option<String>,
}

impl OrdersDeletePurchasesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersDeletePurchasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersDeletePurchasesResponseBuilder::id)
    pub fn build(self) -> Result<OrdersDeletePurchasesResponse, BuildError> {
        Ok(OrdersDeletePurchasesResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
