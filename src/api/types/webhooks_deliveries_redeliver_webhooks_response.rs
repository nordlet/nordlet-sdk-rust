pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeliveriesRedeliverWebhooksResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
}

impl DeliveriesRedeliverWebhooksResponse {
    pub fn builder() -> DeliveriesRedeliverWebhooksResponseBuilder {
        <DeliveriesRedeliverWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveriesRedeliverWebhooksResponseBuilder {
    id: Option<String>,
    status: Option<String>,
}

impl DeliveriesRedeliverWebhooksResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeliveriesRedeliverWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeliveriesRedeliverWebhooksResponseBuilder::id)
    /// - [`status`](DeliveriesRedeliverWebhooksResponseBuilder::status)
    pub fn build(self) -> Result<DeliveriesRedeliverWebhooksResponse, BuildError> {
        Ok(DeliveriesRedeliverWebhooksResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
