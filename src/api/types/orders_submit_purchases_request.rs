pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersSubmitPurchasesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl OrdersSubmitPurchasesRequest {
    pub fn builder() -> OrdersSubmitPurchasesRequestBuilder {
        <OrdersSubmitPurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersSubmitPurchasesRequestBuilder {
    id: Option<String>,
    reason: Option<String>,
}

impl OrdersSubmitPurchasesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersSubmitPurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersSubmitPurchasesRequestBuilder::id)
    pub fn build(self) -> Result<OrdersSubmitPurchasesRequest, BuildError> {
        Ok(OrdersSubmitPurchasesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason,
        })
    }
}
