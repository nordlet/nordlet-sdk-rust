pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersCancelEcommerceRequest {
    #[serde(default)]
    pub id: String,
}

impl OrdersCancelEcommerceRequest {
    pub fn builder() -> OrdersCancelEcommerceRequestBuilder {
        <OrdersCancelEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersCancelEcommerceRequestBuilder {
    id: Option<String>,
}

impl OrdersCancelEcommerceRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersCancelEcommerceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersCancelEcommerceRequestBuilder::id)
    pub fn build(self) -> Result<OrdersCancelEcommerceRequest, BuildError> {
        Ok(OrdersCancelEcommerceRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
