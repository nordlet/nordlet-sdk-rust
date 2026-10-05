pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeliveriesRedeliverWebhooksRequest {
    #[serde(default)]
    pub id: String,
}

impl DeliveriesRedeliverWebhooksRequest {
    pub fn builder() -> DeliveriesRedeliverWebhooksRequestBuilder {
        <DeliveriesRedeliverWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveriesRedeliverWebhooksRequestBuilder {
    id: Option<String>,
}

impl DeliveriesRedeliverWebhooksRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeliveriesRedeliverWebhooksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeliveriesRedeliverWebhooksRequestBuilder::id)
    pub fn build(self) -> Result<DeliveriesRedeliverWebhooksRequest, BuildError> {
        Ok(DeliveriesRedeliverWebhooksRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
