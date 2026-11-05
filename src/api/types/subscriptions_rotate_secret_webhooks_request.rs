pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionsRotateSecretWebhooksRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl SubscriptionsRotateSecretWebhooksRequest {
    pub fn builder() -> SubscriptionsRotateSecretWebhooksRequestBuilder {
        <SubscriptionsRotateSecretWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionsRotateSecretWebhooksRequestBuilder {
    id: Option<String>,
    secret: Option<String>,
}

impl SubscriptionsRotateSecretWebhooksRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionsRotateSecretWebhooksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubscriptionsRotateSecretWebhooksRequestBuilder::id)
    pub fn build(self) -> Result<SubscriptionsRotateSecretWebhooksRequest, BuildError> {
        Ok(SubscriptionsRotateSecretWebhooksRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            secret: self.secret,
        })
    }
}
