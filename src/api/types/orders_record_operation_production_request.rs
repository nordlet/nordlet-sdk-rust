pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersRecordOperationProductionRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "actualMinutes")]
    #[serde(default)]
    pub actual_minutes: String,
}

impl OrdersRecordOperationProductionRequest {
    pub fn builder() -> OrdersRecordOperationProductionRequestBuilder {
        <OrdersRecordOperationProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersRecordOperationProductionRequestBuilder {
    id: Option<String>,
    actual_minutes: Option<String>,
}

impl OrdersRecordOperationProductionRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn actual_minutes(mut self, value: impl Into<String>) -> Self {
        self.actual_minutes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersRecordOperationProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersRecordOperationProductionRequestBuilder::id)
    /// - [`actual_minutes`](OrdersRecordOperationProductionRequestBuilder::actual_minutes)
    pub fn build(self) -> Result<OrdersRecordOperationProductionRequest, BuildError> {
        Ok(OrdersRecordOperationProductionRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            actual_minutes: self
                .actual_minutes
                .ok_or_else(|| BuildError::missing_field("actual_minutes"))?,
        })
    }
}
