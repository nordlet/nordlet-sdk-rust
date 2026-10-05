pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersClosePurchasesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl OrdersClosePurchasesRequest {
    pub fn builder() -> OrdersClosePurchasesRequestBuilder {
        <OrdersClosePurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersClosePurchasesRequestBuilder {
    id: Option<String>,
    reason: Option<String>,
}

impl OrdersClosePurchasesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersClosePurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersClosePurchasesRequestBuilder::id)
    pub fn build(self) -> Result<OrdersClosePurchasesRequest, BuildError> {
        Ok(OrdersClosePurchasesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason,
        })
    }
}
